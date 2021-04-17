import os
import time
import logging
from dlog_python import DlogLogger


logger = logging.getLogger('main')
logger.setLevel(logging.DEBUG)
logger.addHandler(DlogLogger("wrong"))

logger.info("Logging")