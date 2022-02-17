import base64
import json
from PIL import Image, UnidentifiedImageError


def print_type(text: str, data):
    """
    Prints text,type(data),data
    """
    print(text, ": ", type(data), " ", str(data))


def print_info(text: str, data):
    print(text, ": ", str(data).replace('\n', ' '))


def bytes_to_dict(received: bytes):
    """
    Converts bytes object to dict.

    :param received: bytes
    :return: dict
    """
    to_handle = received.decode('utf-8')[1:-1].split("}{")
    translateds = []
    for block in to_handle:
        block = '{' + block + '}'
        try:
            translateds.append(json.loads(block))
        except json.decoder.JSONDecodeError as e:
            print_info('bytes_to_dict', str(e))
            print("bytes_to_dict1: ", translateds)
            print("bytes_to_dict2: ", block)
            print("bytes_to_dict3: ", json.loads(block))
            translateds.append(to_dict("bytes_to_dict: ", str(received)))
        return translateds


def dict_to_bytes(to_send):
    """
    Converts dict to bytes

    :param to_send:dict
    :return: bytes
    """
    if type(to_send) != dict:
        raise Exception("Parameter should be <class 'dict'> type not " + str(type(to_send)))
    return json.dumps(to_send).encode('utf-8')


def to_dict(key: str, value):
    """
    Returns dictionary from key and value

    :param key:
    :param value:
    :return: {key:value}
    """
    return dict({key: value})


def send(response: dict, connection_socket):
    try:
        connection_socket(dict_to_bytes(response))
        print_type("Network.send", response)
    except ConnectionResetError as e:
        print_info("Network.send.reset", e)

def picture_to_string(path):
    try:
        a = Image.open(path).convert('RGBA')
    except AttributeError:
        return False
    except UnidentifiedImageError:
        return False
    b = a.resize((100, 100))
    c = b.tobytes()
    d = base64.b64encode(c)
    e = d.decode()
    return e


def string_to_image(photo_string):
    a = photo_string.encode()
    b = base64.b64decode(a)
    c = Image.frombytes('RGBA', (100, 100), b)
    return c
