import logging
import time
from dlog_python import DlogLogger


logger = logging.getLogger('main')
logger.setLevel(logging.DEBUG)
logger.addHandler(DlogLogger("997a1c6f-4fff-4a7e-b399-74ac397a4fec"))

logger.info("Test")