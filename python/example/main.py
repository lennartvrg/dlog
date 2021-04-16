import logging
from dlog_python import DlogLogger


class Main:
    def __init__(self):
        self.logger = logging.getLogger('main')
        self.logger.setLevel(logging.INFO)
        self.logger.addHandler(DlogLogger("997a1c6f-4fff-4a7e-b399-74ac397a4fec"))

    def run(self):
        while True:
            log = input("> ")
            self.logger.info(log)


if __name__ == "__main__":
    main = Main()
    main.run()