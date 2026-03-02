import marnd_py as mrnd
from marnd_py import PyMPRng 


cfg = {
    "schema": "Lcg64::SV",
    "seed": "123",
}

rng = mrnd.create_rng(cfg);
for i in range(10):
    print(f"#{i}, {rng.next_u64():x}")








