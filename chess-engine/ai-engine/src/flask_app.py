import json
from typing import Union

from flask import Flask, json as fson
from flask_cors import CORS

from model.dto.DTO import DTO
from model.dto.DtoUtil import DtoUtil

app = Flask(__name__)
cors = CORS(app, resources={r"/*": {"origins": "*"}})


def to_json_response(data: Union[dict, DTO], **kwargs: any):
    return fson.jsonify(json.loads(DtoUtil.to_json_str(data)), **kwargs)
