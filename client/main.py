import sys
import os

sys.path.insert(0, os.getcwd() + os.sep + "proto")

import grpc

import proto.hello_pb2 as hello
import proto.hello_pb2_grpc as hello_grpc

def run():
    with grpc.insecure_channel("[::1]:50051") as channel:
        stub = hello_grpc.HelloStub(channel)
        message = hello.HelloRequest(name="Lewin")
        response = stub.SendHello(message)
        
    print(response)

if __name__ == '__main__':
    run()