from typing import Tuple, List
import torch
from torch.nn import Module
from torch.optim import Optimizer
from torch.utils.data import DataLoader


def train(
    net: Module,
    epoch: int,
    train_loader: DataLoader,
    validation_loader: DataLoader,
    criterion: Module,
    optimizer: Optimizer,
    device
) -> Tuple[List[float], List[int], List[float], List[int]]:
    train_loss = []
    train_accuracy = []
    validation_loss = []
    validation_accuracy = []

    for i in range(epoch):
        print('epoch %d/%d' % (i + 1, epoch))
        net.train()

        for data, label in train_loader:
            optimizer.zero_grad()
            output = net(data.to(device))
            label = label.to(device)
            loss = criterion(output, label)
            loss.backward()
            optimizer.step()
            train_loss.append(loss.item())
            train_accuracy.append((output.max(1)[1] == label).sum().item())

        net.eval()

        with torch.no_grad():
            for data, label in validation_loader:
                output = net(data.to(device))
                label = label.to(device)
                validation_loss.append(criterion(output, label).item())
                validation_accuracy.append((output.max(1)[1] == label).sum().item())

    return train_loss, train_accuracy, validation_loss, validation_accuracy
