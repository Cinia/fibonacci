import sys

def fib(i):
    if i < 2:
        return 1
    return fib(i-1) + fib(i-2)

if __name__ == "__main__":
    print(fib(int(sys.argv[1])))
