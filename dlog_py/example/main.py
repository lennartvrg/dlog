import os
import time

from dlog_py import with_dlog


@with_dlog(os.environ['DLOG_API_KEY'])
def example(logger, counter):
    logger.warning(f"Hello World! 3714 4963 5398 431 {counter}")


counter = 0
while True:
    example(counter)
    counter += 1
    time.sleep(0.5)
