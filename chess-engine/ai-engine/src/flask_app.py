import json
from typing import Union

from flask import Flask, json as fson
from flask_cors import CORS

from model.dto.DTO import DTO
from model.dto.DtoUtil import DtoUtil
from model.dto.NumpyEncoder import NumpyJSONEncoder

app = Flask(__name__)
cors = CORS(app, resources={r"/*": {"origins": "*"}})


def to_json_response(data: Union[dict, DTO], **kwargs: any):
    # if isinstance(data, DTO):
    #     return fson.jsonify(json.dumps(data.to_json(), cls=NumpyJSONEncoder), **kwargs)
    # else:
    return fson.jsonify(json.loads(json.dumps(data, cls=NumpyJSONEncoder)))
