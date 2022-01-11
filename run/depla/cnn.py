from typing import Any
from torch import Tensor
from torch.nn import Module, Conv2d, BatchNorm2d, Linear
from torch.nn.functional import relu


class CNN(Module):
    def __init__(self, depth: int, middle_channels: int, middle_features: int, device: Any):
        super(CNN, self).__init__()

        def create_conv(i: int):
            return Conv2d(2 if i == 0 else middle_channels, middle_channels, 3, padding=1, device=device)

        def create_batch():
            return BatchNorm2d(middle_channels, device=device)

        self.__blocks = tuple((create_conv(i), create_batch()) for i in range(depth))
        self.__fc1 = Linear(middle_channels * 64, middle_features, device=device)
        self.__fc2 = Linear(middle_features, 64, device=device)

    def forward(self, x: Tensor):
        for conv, batch in self.__blocks:
            x = relu(batch(conv(x)))
        x = x.flatten(1)
        x = self.__fc1(x)
        x = relu(x)
        x = self.__fc2(x)
        x = x.softmax(1)
        x = x.view(-1, 8, 8)
        return x
