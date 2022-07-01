import os
import sys
from typing import NoReturn

scripture_path = os.sep.join(os.path.abspath(__file__).split(os.sep)[:-2]) + os.sep + 'Scripture.md'


def validate_environment() -> None | NoReturn:
    if len(sys.argv) != 1:
        print('This program takes no arguments, found:', sys.argv)
        print('Exiting...')
        exit(1)

    if not os.path.exists(scripture_path):
        print('The scripture does not exist! Exiting...')
        exit(1)

    if os.path.isdir(scripture_path):
        print("At the scripture's path is a directory. Exiting...")
        exit(1)


def main() -> None:
    validate_environment()

    with open(scripture_path, 'r') as file:
        print('START OF SCRIPTURE\n')

        for line in file.read().splitlines():
            line = line.lstrip('#').lstrip()
            print(line)

    print('\nEND OF SCRIPTURE')


if __name__ == '__main__':
    main()
