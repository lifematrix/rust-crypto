import marcrypto as mc

cfg = {
    "schema": "Lcg64::DK",
    "seed": "123",
}

rng = mc.random.from_config(cfg);
for i in range(10):
    print(f"#{i}, {rng.next_u64():x}")

print("-"*80)

rng = mc.random.rng_dk();
for i in range(10):
    print(f"#{i}, {rng.next_u64():x}")








