import sys

sys.path.append('run/python')

from dataset import WTHORDataset
from ai import AI
from torch.utils.data import DataLoader


def get_ai():
    device = "cuda"
    dataset = WTHORDataset(device, ['run/wthor/WTH_%d.wtb' % year for year in range(2010, 2021)])
    loader = DataLoader(dataset, 1024, True, drop_last=True)
    ai = AI(device, 4, loader)
    return ai
