#include <fstream>
#include <iostream>
#include <sstream>
#include <string>

enum class State {
  Normal,
  SawSlash,
  SawStar,
  InLineComment,
  InBlockComment,
  InBlockCommentStar,
  InString,
  InChar
};

static bool transform_to_stream(std::istream &in, std::ostream &out,
                                std::string &err_msg) {
  State st = State::Normal;
  char c = '\0';

  long long line = 1;
  long long block_start_line = -1;

  auto advance_line = [&](char ch) {
    if (ch == '\n')
      line++;
  };

  while (in.get(c)) {
    switch (st) {
    case State::Normal:
      if (c == '/') {
        st = State::SawSlash;
      } else if (c == '*') {
        st = State::SawStar;
      } else if (c == '"') {
        out.put(c);
        st = State::InString;
      } else if (c == '\'') {
        out.put(c);
        st = State::InChar;
      } else {
        out.put(c);
        advance_line(c);
      }
      break;

    case State::SawSlash:
      if (c == '*') {
        st = State::InBlockComment;
        block_start_line = line;
        out.put(' ');
        out.put(' ');
      } else if (c == '/') {
        st = State::InLineComment;
        out.put(' ');
        out.put(' ');
      } else {
        out.put('/');
        if (c == '"') {
          out.put(c);
          st = State::InString;
        } else if (c == '\'') {
          out.put(c);
          st = State::InChar;
        } else {
          out.put(c);
          advance_line(c);
          st = State::Normal;
        }
      }
      break;

    case State::SawStar:
      if (c == '/') {
        // err_msg = "ERROR: Program contains C-style, unexpected comment "
        //           "terminator on line " +
        //           std::to_string(line);
        err_msg =
            "ERROR: Program contains C-style, unterminated comment on line " +
            std::to_string(line);

        return false;
      } else {
        out.put('*');
        if (c == '*') {
          st = State::SawStar;
        } else if (c == '"') {
          out.put(c);
          st = State::InString;
        } else if (c == '\'') {
          out.put(c);
          st = State::InChar;
        } else {
          out.put(c);
          advance_line(c);
          st = State::Normal;
        }
      }
      break;

    case State::InLineComment:
      if (c == '\n') {
        out.put('\n');
        line++;
        st = State::Normal;
      } else {
        out.put(' ');
      }
      break;

    case State::InBlockComment:
      if (c == '*') {
        out.put(' ');
        st = State::InBlockCommentStar;
      } else if (c == '\n') {
        out.put('\n');
        line++;
      } else {
        out.put(' ');
      }
      break;

    case State::InBlockCommentStar:
      if (c == '/') {
        out.put(' ');
        st = State::Normal;
        block_start_line = -1;
      } else if (c == '*') {
        out.put(' ');
      } else if (c == '\n') {
        out.put('\n');
        line++;
        st = State::InBlockComment;
      } else {
        out.put(' ');
        st = State::InBlockComment;
      }
      break;

    case State::InString:
      out.put(c);
      advance_line(c);
      if (c == '\\') {
        if (in.get(c)) {
          out.put(c);
          advance_line(c);
        }
      } else if (c == '"') {
        st = State::Normal;
      }
      break;

    case State::InChar:
      out.put(c);
      advance_line(c);
      if (c == '\\') {
        if (in.get(c)) {
          out.put(c);
          advance_line(c);
        }
      } else if (c == '\'') {
        st = State::Normal;
      }
      break;
    }
  }

  if (st == State::SawSlash) {
    out.put('/');
  } else if (st == State::SawStar) {
    out.put('*');
  } else if (st == State::InBlockComment || st == State::InBlockCommentStar) {
    err_msg = "ERROR: Program contains C-style, unterminated comment on line " +
              std::to_string(block_start_line);
    return false;
  }

  return true;
}

int main(int argc, char *argv[]) {
  if (argc != 2) {
    std::cerr << "Usage: " << argv[0] << " <input_file>\n";
    return 1;
  }

  std::string filename = argv[1];
  std::string out_filename = filename.substr(0, filename.size() - 2) +
                             "-comments_replaced_with_whitespace.c";

  std::ifstream in(filename);
  if (!in) {
    std::cerr << "ERROR: Could not open file: " << filename << "\n";
    return 1;
  }

  std::ostringstream buffer;
  std::string err;

  if (!transform_to_stream(in, buffer, err)) {
    std::cerr << err << "\n";
    return 1;
  }

  std::ofstream out(out_filename);
  if (!out) {
    std::cerr << "ERROR: Could not open output file: " << out_filename << "\n";
    return 1;
  }

  out << buffer.str();
  return 0;
}
