import os
from subprocess import run
if __name__ == '__main__':
    for d in os.listdir(os.getcwd()):
        if d != 'compile.py':
            print("going to compile {}".format(d))
            os.chdir(r"{}".format(d))
            run(["cargo", "update"])
            run(["cargo", "rustc", "--release", "--", "-C", "target-cpu=native", "-C", "lto"])
            os.chdir('..')