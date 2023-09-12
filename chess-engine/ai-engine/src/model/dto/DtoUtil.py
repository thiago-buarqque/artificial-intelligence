import json
from typing import Union

import numpy as np

from model.dto.DTO import DTO





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
            return json.dumps(data.to_json(), cls=NumpyJSONEncoder)
        else:
            return json.dumps(data, cls=NumpyJSONEncoder)
