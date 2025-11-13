import time
import sys
import pyautogui as pag

pag.FAILSAFE = True

def main():
    last = None
    try:
        while True:
            x, y = pag.position()
            if last != (x, y):
                try:
                    r, g, b = pag.pixel(x, y)
                except Exception:
                    r = g = b = float("nan")

                sys.stdout.write(f"\rX: {x:4d}  Y: {y:4d}  RGB: ({int(r) if r==r else 'NaN'}, "
                                 f"{int(g) if g==g else 'NaN'}, {int(b) if b==b else 'NaN'})")
                sys.stdout.flush()
                last = (x, y)
            time.sleep(1)
    except KeyboardInterrupt:
        print("\nBye!")

if __name__ == "__main__":
    main()
