#!/usr/bin/env python
from __future__ import division

from random import randrange

import numpy as np




class Character(object):
    def __init__(self, str, dex, d):
        self.str = str
        self.dex = dex
        self.d = d

    @staticmethod
    def rand():
        points = 500

        str, dex, d = np.random.dirichlet(np.ones(3)) * points
        str = int(str)
        dex = int(dex)
        d = int(d)
        over = points - str - dex - d
        for i in range(0, over):
            c = randrange(3)
            if c == 0:
                str += 1
            elif c == 1:
                dex += 1
            elif c == 2:
                d += 1
            else:
                raise RuntimeError('c == %s' % c)
        str += 10
        dex += 10
        d += 10
        return Character(str=str, dex=dex, d=d)

    def __repr__(self):
        return '%s(%s)' % (self.__class__.__name__, ', '.join('%s=%s' % (k, v) for k, v in self.__dict__.items()))


def dmg_bonus(str, d):
    return str / (str + d)


def crit_chance(dex, d):
    return dex / (dex + 3 * d)

crit_multiplier = 5
def crit_bonus(dex, d):
    # print 'crit_chance:', crit_chance(dex, d)
    # print 'crit_bonus:', 1 + crit_chance(dex, d) * crit_multiplier
    return 1 + crit_chance(dex, d) * crit_multiplier


def dps(attacker, defender):
    # print 'dmg_bonus:', dmg_bonus(attacker.str, defender.d)
    dps = dmg_bonus(attacker.str, defender.d) * crit_bonus(attacker.dex, defender.d)
    # print 'dps', dps
    return dps


def battle(a, b):
    dpsa = dps(a, b)
    dpsb = dps(b, a)
    if dpsa > 1.5 * dpsb:
        return True
    if dpsb > 1.5 * dpsa:
        return False
    return None
