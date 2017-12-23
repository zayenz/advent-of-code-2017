h = 0
f = 0
d = 0
e = 0
a = 1
b = 92100
c = 109100
for b in range(92100, 109100, 17):
    f = 1
    d = 2
    while d != b:
        if b % d == 0:
            f = 0
        d += 1
    if f == 0:
        h -= 1
print(f"value is {h}")
