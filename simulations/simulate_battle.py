#!/usr/bin/env python
import pandas as pd

from miran import Character, battle

cs = [Character.rand() for i in range(0, 200)]


df = pd.DataFrame(((c.str, c.dex, c.d) for c in cs), columns=['str', 'dex', 'def'])
df['wins'] = 0

for i in range(0, len(cs)):
    for j in range(i + 1, len(cs)):
        assert i != j
        result = battle(cs[i], cs[j])
        if result is None:
            continue
        if result:
            df.ix[i, 'wins'] += 1
        else:
            df.ix[j, 'wins'] += 1

print df.sort_values('wins', ascending=False)
