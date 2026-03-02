# python/test_marcrypto_random.py
import sys
import traceback

import marcrypto


def assert_eq(a, b, msg=""):
    if a != b:
        raise AssertionError(msg or f"Assertion failed: {a!r} != {b!r}")


def test_import_and_metadata():
    print("== test_import_and_metadata ==")
    print("marcrypto.__version__ =", getattr(marcrypto, "__version__", None))
    print("marcrypto.__author__  =", getattr(marcrypto, "__author__", None))
    assert getattr(marcrypto, "random", None) is not None, "marcrypto.random submodule missing"


def test_default_rng_works():
    print("== test_default_rng_works ==")
    rng = marcrypto.random.default_rng()
    x = rng.next_u64()
    print("default_rng().next_u64() =", x)
    assert isinstance(x, int)
    assert 0 <= x < (1 << 64)


def test_seed_determinism_default_rng():
    print("== test_seed_determinism_default_rng ==")
    r1 = marcrypto.random.default_rng(123)
    r2 = marcrypto.random.default_rng(123)

    seq1 = [r1.next_u64() for _ in range(10)]
    seq2 = [r2.next_u64() for _ in range(10)]
    print("seq1 =", seq1)
    print("seq2 =", seq2)
    assert_eq(seq1, seq2, "Same seed should produce identical sequences")


def test_seed_changes_sequence():
    print("== test_seed_changes_sequence ==")
    r1 = marcrypto.random.default_rng(123)
    r2 = marcrypto.random.default_rng(124)

    seq1 = [r1.next_u64() for _ in range(5)]
    seq2 = [r2.next_u64() for _ in range(5)]
    print("seed=123:", seq1)
    print("seed=124:", seq2)

    # Not a mathematical proof, but should almost always differ quickly.
    if seq1 == seq2:
        raise AssertionError("Different seeds unexpectedly produced identical sequences")


def test_rng_variants_construct():
    print("== test_rng_variants_construct ==")
    r_dk = marcrypto.random.rng_dk(1)
    r_sv = marcrypto.random.rng_sv(1)
    r_pcg = marcrypto.random.rng_pcg64(1)

    x_dk = r_dk.next_u64()
    x_sv = r_sv.next_u64()
    x_pcg = r_pcg.next_u64()

    print("dk =", x_dk)
    print("sv =", x_sv)
    print("pcg64 =", x_pcg)

    # Usually these presets differ; if your DK and SV are identical presets,
    # this may fail and you can weaken/remove it.
    if x_dk == x_sv == x_pcg:
        raise AssertionError("All presets produced the same first output (unexpected)")


def test_from_config_minimal():
    print("== test_from_config_minimal ==")
    # Note: your Rust expects dict[str, str], so seed must be a string.
    rng = marcrypto.random.from_config({"schema": "Lcg64::DK", "seed": "123"})
    x = rng.next_u64()
    print("from_config(DK, seed=123).next_u64() =", x)
    assert isinstance(x, int)
    assert 0 <= x < (1 << 64)


def test_from_config_matches_default_rng_seeded():
    print("== test_from_config_matches_default_rng_seeded ==")
    r1 = marcrypto.random.default_rng(123)
    r2 = marcrypto.random.from_config({"schema": "Lcg64::DK", "seed": "123"})

    seq1 = [r1.next_u64() for _ in range(10)]
    seq2 = [r2.next_u64() for _ in range(10)]
    print("default_rng(123) =", seq1)
    print("from_config DK   =", seq2)
    assert_eq(seq1, seq2, "from_config should match default_rng for same schema+seed")


def test_from_config_bad_types():
    print("== test_from_config_bad_types ==")
    try:
        # seed is int, but your binding currently requires dict[str, str]
        marcrypto.random.from_config({"schema": "Lcg64::DK", "seed": 123})
    except Exception as e:
        print("expected error:", type(e).__name__, e)
    else:
        raise AssertionError("Expected from_config to fail when seed is not a string")


def run_all():
    tests = [
        test_import_and_metadata,
        test_default_rng_works,
        test_seed_determinism_default_rng,
        test_seed_changes_sequence,
        test_rng_variants_construct,
        test_from_config_minimal,
        test_from_config_matches_default_rng_seeded,
        test_from_config_bad_types,
    ]

    ok = 0
    for t in tests:
        try:
            t()
            ok += 1
        except Exception:
            print(f"\nFAILED: {t.__name__}")
            traceback.print_exc()
            sys.exit(1)

    print(f"\nAll tests passed ({ok}/{len(tests)}).")


if __name__ == "__main__":
    run_all()