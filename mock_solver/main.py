import socket as st
import struct

from bayes_opt import BayesianOptimization


MSGLEN = 8


def get_data(socket):
    data = socket.recv(MSGLEN)
    return struct.unpack('d', data)[0]
    # chunks = []
    # bytes_recd = 0
    # while bytes_recd < MSGLEN:
    #     chunk = socket.recv(min(MSGLEN - bytes_recd, 2048))
    #     if chunk == b'':
    #         raise RuntimeError("socket connection broken")
    #     chunks.append(chunk)
    #     bytes_recd = bytes_recd + len(chunk)
    # return b''.join(chunks)


def send_coordinate(socket, x, y, z):
    socket.send(x)
    socket.send(y)
    socket.send(z)
    # totalsent = 0
    # while totalsent < MSGLEN:
    #    sent = socket.send(msg[totalsent:])
    #    if sent == 0:
    #        raise RuntimeError("socket connection broken")
    #    totalsent = totalsent + sent


class Updater():
    def __init__(self, socket):
        self._socket = socket

    def update(self, x, y, z):
        print("Sending")
        send_coordinate(self._socket, x, y, z)
        print("Recieving")
        data = get_data(self._socket)
        #data = x + y + z
        print("Data: {0}".format(data))
        return data


def run(socket):
    updater = Updater(socket)
    pbounds = {"x": (-800, -740), "y": (330, 340), "z": (220, 224)}
    optimiser = BayesianOptimization(
        f=updater.update, pbounds=pbounds, random_state=1)
    optimiser.maximize(init_points=2, n_iter=30)
    # while True:
    #     coord_text = input("Coord: ")
    #     splits = coord_text.split(",")
    #     a = struct.pack('d', float(splits[0]))
    #     b = struct.pack('d', float(splits[1]))
    #     c = struct.pack('d', float(splits[2]))

    #     send_coordinate(socket, a, b, c)

    #     print(get_data(socket))


s = st.socket()
port = 34892
s.connect(('127.0.0.1', port))
run(s)
#data = get_data(s)
s.close()
