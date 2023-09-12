import json
from abc import ABC

import numpy as np

from model.dto.NumpyEncoder import NumpyJSONEncoder


class DTO(ABC, dict):
    pass
    # def to_json(self) -> str:
    #     def dumper(obj):
    #         try:
    #             return obj.to_json()
    #         except:
    #             return obj.__dict__
    #
    #     return json.dumps(self, default=dumper,
    #                       sort_keys=True, indent=4, cls=NumpyJSONEncoder)

