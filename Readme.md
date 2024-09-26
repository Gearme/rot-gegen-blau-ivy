[...]/move?field=
{"field":"BRRBooo","state":"success"}


- params
    - l: remaining length of field
- 1 o
    - gewonnen (lege 1)
- 2 oo
    - gewonnen (lege 2)
- 3 ooo
    - immer verloren
- 4 oooo
    - immer verloren
- 5 ooooo
    - lege 1
- 6 oooooo
    - lege 2
- 7
    - lege 1 - nächster zug ist l=5 (gewonnen) oder 4 (verloren)
    - aber besser geht nicht
- 8 
    - lege 1


- l <= 2: lege l
- alle drei felder sind gewinnfelder
    - n (trivial)
    - n-3 (weil zugzwang)


