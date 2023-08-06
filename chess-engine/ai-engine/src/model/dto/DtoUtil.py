import json
from typing import Union

import numpy as np

from model.dto.DTO import DTO


class NumpyJSONEncoder(json.JSONEncoder):
    def default(self, obj):
        if isinstance(obj, np.integer):
            return int(obj)
        if isinstance(obj, np.floating):
            return float(obj)
        if isinstance(obj, np.ndarray):
            return obj.tolist()
        if isinstance(obj, np.bool_):
            return bool(obj)

        return json.JSONEncoder.default(self, obj)


# class DTOEncoder(NumpyJSONEncoder):
#     def default(self, obj):
#         if isinstance(obj, DTO):
#             return json.JSONEncoder.default(self,
#                                             DtoUtil.to_json_str(obj.to_dict()))
#
#         return json.JSONEncoder.default(self, obj)


class DtoUtil:
    @staticmethod
    def to_json_str(data: Union[dict, DTO]):
        if isinstance(data, DTO):
            return json.dumps(data.to_dict(), cls=NumpyJSONEncoder)
        else:
            return json.dumps(data, cls=NumpyJSONEncoder)
