from typing import Any
from torch import Tensor
from torch.nn import Module, Conv2d, BatchNorm2d
from torch.nn.functional import relu


class CNN(Module):
    def __init__(self, device: Any):
        super(CNN, self).__init__()
        self.__conv1: Conv2d = Conv2d(2, 128, 1, device=device)
        self.__batch1: BatchNorm2d = BatchNorm2d(128, device=device)
        self.__conv21: Conv2d = Conv2d(128, 128, 3, padding=1, device=device)
        self.__batch21: BatchNorm2d = BatchNorm2d(128, device=device)
        self.__conv22: Conv2d = Conv2d(128, 128, 3, padding=1, device=device)
        self.__batch22: BatchNorm2d = BatchNorm2d(128, device=device)
        self.__conv23: Conv2d = Conv2d(128, 128, 3, padding=1, device=device)
        self.__batch23: BatchNorm2d = BatchNorm2d(128, device=device)
        self.__conv24: Conv2d = Conv2d(128, 128, 3, padding=1, device=device)
        self.__batch24: BatchNorm2d = BatchNorm2d(128, device=device)
        self.__conv25: Conv2d = Conv2d(128, 128, 3, padding=1, device=device)
        self.__batch25: BatchNorm2d = BatchNorm2d(128, device=device)
        self.__conv26: Conv2d = Conv2d(128, 128, 3, padding=1, device=device)
        self.__batch26: BatchNorm2d = BatchNorm2d(128, device=device)
        self.__conv27: Conv2d = Conv2d(128, 128, 3, padding=1, device=device)
        self.__batch27: BatchNorm2d = BatchNorm2d(128, device=device)
        self.__conv28: Conv2d = Conv2d(128, 128, 3, padding=1, device=device)
        self.__batch28: BatchNorm2d = BatchNorm2d(128, device=device)
        self.__conv3: Conv2d = Conv2d(128, 1, 1, device=device)

    def forward(self, x: Tensor) -> Tensor:
        x = relu(self.__batch1(self.__conv1(x)))
        x = relu(self.__batch21(self.__conv21(x)))
        x = relu(self.__batch22(self.__conv22(x)))
        x = relu(self.__batch23(self.__conv23(x)))
        x = relu(self.__batch24(self.__conv24(x)))
        x = relu(self.__batch25(self.__conv25(x)))
        x = relu(self.__batch26(self.__conv26(x)))
        x = relu(self.__batch27(self.__conv27(x)))
        x = relu(self.__batch28(self.__conv28(x)))
        x = self.__conv3(x)
        return x.flatten(1)
