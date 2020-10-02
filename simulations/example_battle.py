#!/usr/bin/env python
import pandas as pd

from miran import Character, battle


print battle(Character(20, 10, 25), Character(15, 15, 25))

