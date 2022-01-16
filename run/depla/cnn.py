from enum import IntEnum
from torch import Tensor
from torch.nn import Module, Sequential, Conv2d, BatchNorm2d, ReLU, Flatten


class Depth(IntEnum):
    Four = 4
    Six = 6
    Eight = 8


class CNN(Module):
    def __init__(self, depth: Depth, device):
        super(CNN, self).__init__()
        sequential = [Conv2d(2, 128, 1, device=device), BatchNorm2d(128, device=device), ReLU()]

        for _ in range(0, int(depth)):
            sequential += [Conv2d(128, 128, 3, padding=1, device=device), BatchNorm2d(128, device=device), ReLU()]

        sequential += [Conv2d(128, 1, 1, device=device), Flatten()]
        self.__block: Sequential = Sequential(*sequential)

    def forward(self, x: Tensor) -> Tensor:
        return self.__block(x)
