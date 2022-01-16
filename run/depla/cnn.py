from enum import IntEnum, auto
from torch import Tensor
from torch.nn import Module, Sequential, Conv2d, BatchNorm2d, ReLU, Flatten


class Depth(IntEnum):
    Four = auto()
    Six = auto()
    Eight = auto


class CNN(Module):
    def __init__(self, depth: Depth, device):
        super(CNN, self).__init__()
        sequential = [Conv2d(2, 128, 1, device=device), BatchNorm2d(128, device=device), ReLU()]

        if depth >= Depth.Four:
            sequential.append(Conv2d(128, 128, 3, padding=1, device=device))
            sequential.append(BatchNorm2d(128, device=device))
            sequential.append(ReLU())
            sequential.append(Conv2d(128, 128, 3, padding=1, device=device))
            sequential.append(BatchNorm2d(128, device=device))
            sequential.append(ReLU())
            sequential.append(Conv2d(128, 128, 3, padding=1, device=device))
            sequential.append(BatchNorm2d(128, device=device))
            sequential.append(ReLU())
            sequential.append(Conv2d(128, 128, 3, padding=1, device=device))
            sequential.append(BatchNorm2d(128, device=device))
            sequential.append(ReLU())

        if depth >= Depth.Six:
            sequential.append(Conv2d(128, 128, 3, padding=1, device=device))
            sequential.append(BatchNorm2d(128, device=device))
            sequential.append(ReLU())
            sequential.append(Conv2d(128, 128, 3, padding=1, device=device))
            sequential.append(BatchNorm2d(128, device=device))
            sequential.append(ReLU())

        if depth >= Depth.Eight:
            sequential.append(Conv2d(128, 128, 3, padding=1, device=device))
            sequential.append(BatchNorm2d(128, device=device))
            sequential.append(ReLU())
            sequential.append(Conv2d(128, 128, 3, padding=1, device=device))
            sequential.append(BatchNorm2d(128, device=device))
            sequential.append(ReLU())

        sequential.append(Conv2d(128, 1, 1, device=device))
        sequential.append(Flatten())
        self.__sequential: Sequential = Sequential(*sequential)

    def forward(self, x: Tensor) -> Tensor:
        return self.__sequential(x)
