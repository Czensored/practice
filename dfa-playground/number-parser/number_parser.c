#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>

typedef enum { Digit, Sign, Period, Other, EndFile, NumTypes } charType;

typedef struct {
  char *str;
  int numAllocated;
  int numUsed;
} stringData;

#define initStringSize 10

/**
 * @typedef transitionFunc
 * @brief DFA transition function signature.
 *
 * Each transition function:
 * - may update the token buffer (start token, append character, emit token,
 * etc.)
 * - returns the next DFA state
 */
typedef int (*transitionFunc)(char, stringData *);

/**
 * @brief Reset (free) the token buffer.
 * @param data Token buffer structure to clear.
 */
static void resetBuf(stringData *data) {
  if (data->str != NULL) {
    free(data->str);
    data->str = NULL;
  }
  data->numAllocated = 0;
  data->numUsed = 0;
}

/**
 * @brief Append a character to the token buffer, growing it as needed.
 *
 * If the buffer is uninitialized, it will be allocated with initStringSize.
 *
 * @param ch Character to append.
 * @param data Token buffer to modify.
 */
void addChar(char ch, stringData *data) {
  if (data->str == NULL) {
    data->numAllocated = initStringSize;
    data->str = malloc(data->numAllocated * sizeof(char));
    data->numUsed = 0;
  }
  if (data->numUsed == data->numAllocated) {
    char *tmp = malloc(2 * data->numAllocated * sizeof(char));
    for (int i = 0; i < data->numAllocated; i++)
      tmp[i] = data->str[i];
    data->numAllocated *= 2;
    free(data->str);
    data->str = tmp;
  }
  data->str[data->numUsed++] = ch;
}

// Transition functions

/**
 * @brief State 0: outside a number.
 *
 * Ignores the current character (delimiter). Also clears any buffer.
 *
 * @param ch Current input character (unused).
 * @param data Token buffer.
 * @return Next state (always 0).
 */
int s0_ignore(char ch, stringData *data) {
  (void)ch;
  resetBuf(data);
  return 0;
}

/**
 * @brief State 0 on Digit: start an integer token.
 * @param ch Current input digit.
 * @param data Token buffer.
 * @return Next state 2 (integer).
 */
int s0_startDigit(char ch, stringData *data) {
  resetBuf(data);
  addChar(ch, data);
  return 2;
}

/**
 * @brief State 0 on Sign: start a signed token (might become int or float).
 * @param ch '+' or '-'.
 * @param data Token buffer.
 * @return Next state 1 (after sign).
 */
int s0_startSign(char ch, stringData *data) {
  resetBuf(data);
  addChar(ch, data);
  return 1;
}

/**
 * @brief State 0 on Period: start a potential float like ".5".
 *
 * Note: '.' alone is not a valid float; the next character must be a digit.
 *
 * @param ch '.' character.
 * @param data Token buffer.
 * @return Next state 3 (dot seen, need digit).
 */
int s0_startDot(char ch, stringData *data) {
  resetBuf(data);
  addChar(ch, data);
  return 3;
}

/**
 * @brief State 1 (after sign) on Digit: transition into integer state.
 * @param ch Digit.
 * @param data Token buffer.
 * @return Next state 2.
 */
int s1_toInt(char ch, stringData *data) {
  addChar(ch, data);
  return 2;
}

/**
 * @brief State 1 (after sign) on Period: transition to "dot seen" state.
 * Allows numbers like "+.5" or "-.25".
 * @param ch '.'.
 * @param data Token buffer.
 * @return Next state 3.
 */
int s1_toDot(char ch, stringData *data) {
  addChar(ch, data);
  return 3;
}

/**
 * @brief State 1 fail: sign not followed by digit or '.' -> not a number.
 *
 * Discards the buffered sign token.
 *
 * @param ch Current input character (unused).
 * @param data Token buffer.
 * @return Next state 0.
 */
int s1_fail(char ch, stringData *data) {
  (void)ch;
  resetBuf(data);
  return 0;
}

/**
 * @brief State 2: integer digits on Digit -> keep consuming digits.
 * @param ch Digit.
 * @param data Token buffer.
 * @return Next state 2.
 */
int s2_digit(char ch, stringData *data) {
  addChar(ch, data);
  return 2;
}

/**
 * @brief State 2 on Period: integer becomes float (longest match).
 * @param ch '.'.
 * @param data Token buffer.
 * @return Next state 4 (float).
 */
int s2_dot(char ch, stringData *data) {
  addChar(ch, data);
  return 4;
}

/**
 * @brief State 2 done: delimiter/EOF ends the integer.
 *
 * Prints the integer, clears the buffer, and pushes the delimiter back
 * (so it can be processed again from the start state).
 *
 * @param ch Delimiter character (or EOF).
 * @param data Token buffer.
 * @return Next state 0.
 */
int s2_done(char ch, stringData *data) {
  addChar('\0', data);
  printf("Number: Integer\n  %s\n", data->str);
  resetBuf(data);

  if (ch != EOF)
    ungetc(ch, stdin);
  return 0;
}

/**
 * @brief State 3 (dot seen) on Digit: becomes a valid float.
 * @param ch Digit.
 * @param data Token buffer.
 * @return Next state 4.
 */
int s3_needDigit(char ch, stringData *data) {
  addChar(ch, data);
  return 4;
}

/**
 * @brief State 3 invalid: '.' (or '+.'/'-.') not followed by digit.
 *
 * Discards the buffered token. No number is printed because '.' alone
 * is not a float for this assignment.
 *
 * @param ch Current input character (unused).
 * @param data Token buffer.
 * @return Next state 0.
 */
int s3_notNumber(char ch, stringData *data) {
  (void)ch;
  resetBuf(data);
  return 0;
}

/**
 * @brief State 4: float state on Digit -> keep consuming digits.
 * @param ch Digit.
 * @param data Token buffer.
 * @return Next state 4.
 */
int s4_digit(char ch, stringData *data) {
  addChar(ch, data);
  return 4;
}

/**
 * @brief State 4 on Period: second '.' ends the float (longest match).
 *
 * Prints the float, clears the buffer, and pushes '.' back so the next
 * number can start with '.' (e.g., "123.45.678" -> "123.45" and ".678").
 *
 * @param ch '.'.
 * @param data Token buffer.
 * @return Next state 0.
 */
int s4_done_onDot(char ch, stringData *data) {
  addChar('\0', data);
  printf("Number: Float\n  %s\n", data->str);
  resetBuf(data);

  ungetc(ch, stdin);
  return 0;
}

/**
 * @brief State 4 done: delimiter/EOF ends the float.
 *
 * Prints the float, clears the buffer, and pushes the delimiter back
 * (if not EOF) for continued scanning.
 *
 * @param ch Delimiter character (or EOF).
 * @param data Token buffer.
 * @return Next state 0.
 */
int s4_done(char ch, stringData *data) {
  addChar('\0', data);
  printf("Number: Float\n  %s\n", data->str);
  resetBuf(data);

  if (ch != EOF)
    ungetc(ch, stdin);
  return 0;
}

/**
 * @brief Program entry point.
 *
 * Reads characters from stdin until EOF, classifies each character into a
 * charType, and performs DFA transitions using a table of transition functions.
 *
 * @return 0 on normal termination.
 */
int main(void) {
  transitionFunc table[5][NumTypes] = {
      // Digit        Sign         Period       Other        EndFile
      {s0_startDigit, s0_startSign, s0_startDot, s0_ignore, s0_ignore},       // 0
      {s1_toInt, s1_fail, s1_toDot, s1_fail, s1_fail},                        // 1
      {s2_digit, s2_done, s2_dot, s2_done, s2_done},                          // 2
      {s3_needDigit, s3_notNumber, s3_notNumber, s3_notNumber, s3_notNumber}, // 3
      {s4_digit, s4_done, s4_done_onDot, s4_done, s4_done}                    // 4
  };

  stringData paramData = {NULL, 0, 0};
  int currentState = 0;

  int inputCh;
  while ((inputCh = getchar()) != EOF) {
    charType inputType;
    if (isdigit((unsigned char)inputCh))
      inputType = Digit;
    else if (inputCh == '+' || inputCh == '-')
      inputType = Sign;
    else if (inputCh == '.')
      inputType = Period;
    else
      inputType = Other;

    currentState = table[currentState][inputType]((char)inputCh, &paramData);
  }

  currentState = table[currentState][EndFile](EOF, &paramData);

  printf("processing terminated\n");
  return 0;
}
