from typing import List
import torch
from torch.nn import Module
from matplotlib import pyplot


def get_path(parent: str, child: str) -> str:
    return parent + '/' + child


def wthor_paths(path: str, start: int, end: int) -> List[str]:
    return [path % year for year in range(start, end + 1)]


def save_net(path: str, net: Module):
    torch.save(net.state_dict(), path)


def load_net(path: str, net: Module):
    net.load_state_dict(torch.load(path))


def save_png(
    path: str,
    title: str,
    train,
    train_label: str,
    validation,
    validation_label: str,
    dpi: int
):
    pyplot.figure()
    pyplot.title(title)
    pyplot.plot(train, label=train_label)
    pyplot.plot(validation, label=validation_label)
    pyplot.legend()
    pyplot.savefig(path, dpi=dpi)
