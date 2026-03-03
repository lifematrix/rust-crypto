# python/test_choice.py
import sys
import traceback

import marcrypto


def assert_raises(exc_type, fn, *args, **kwargs):
    try:
        fn(*args, **kwargs)
    except exc_type as e:
        return e
    except Exception as e:
        raise AssertionError(f"Expected {exc_type.__name__}, got {type(e).__name__}: {e}") from e
    else:
        raise AssertionError(f"Expected {exc_type.__name__}, but no exception was raised")


def test_choice_idx_basic():
    rng = marcrypto.random.default_rng(123)
    idx = rng.choice_idx([0.2, 0.3, 0.5])
    assert isinstance(idx, int)
    assert 0 <= idx < 3


def test_choice_idx_deterministic():
    r1 = marcrypto.random.default_rng(42)
    r2 = marcrypto.random.default_rng(42)

    p = [0.1, 0.2, 0.7]
    s1 = [r1.choice_idx(p) for _ in range(50)]
    s2 = [r2.choice_idx(p) for _ in range(50)]
    assert s1 == s2, "Same seed should yield identical choice_idx sequence"


def test_choice_basic_strings():
    rng = marcrypto.random.default_rng(123)
    elems = ["a", "b", "c"]
    p = [0.2, 0.3, 0.5]
    x = rng.choice(elems, p)
    assert x in elems


def test_choice_basic_tuples():
    rng = marcrypto.random.default_rng(123)
    elems = [(1, "x"), (2, "y"), (3, "z")]
    p = [0.2, 0.3, 0.5]
    x = rng.choice(elems, p)
    assert x in elems


def test_choice_preserves_identity_for_objects():
    class Box:
        def __init__(self, v):
            self.v = v

    a, b, c = Box(1), Box(2), Box(3)
    elems = [a, b, c]
    p = [0.2, 0.3, 0.5]

    rng = marcrypto.random.default_rng(123)
    x = rng.choice(elems, p)

    # should return the same object reference (identity), not a copy
    assert x is a or x is b or x is c


def test_choice_length_mismatch():
    rng = marcrypto.random.default_rng(123)
    elems = ["a", "b", "c"]
    p = [0.5, 0.5]  # mismatch
    e = assert_raises(ValueError, rng.choice, elems, p)
    print("expected length mismatch error:", e)


def test_choice_idx_bad_sum():
    rng = marcrypto.random.default_rng(123)
    e = assert_raises(ValueError, rng.choice_idx, [0.2, 0.2, 0.2])
    print("expected bad sum error:", e)


def test_choice_idx_negative_prob():
    rng = marcrypto.random.default_rng(123)
    e = assert_raises(ValueError, rng.choice_idx, [0.2, -0.1, 0.9])
    print("expected negative prob error:", e)


def test_choice_elements_not_sequence():
    rng = marcrypto.random.default_rng(123)
    e = assert_raises(ValueError, rng.choice, 12345, [1.0])  # not a sequence
    print("expected non-sequence error:", e)


def test_choice_matches_choice_idx():
    rng = marcrypto.random.default_rng(999)
    elems = ["x", "y", "z"]
    p = [0.1, 0.2, 0.7]

    # Because both calls advance RNG state, compare using two RNGs with same seed:
    r1 = marcrypto.random.default_rng(999)
    r2 = marcrypto.random.default_rng(999)

    idx = r1.choice_idx(p)
    x = r2.choice(elems, p)
    assert x == elems[idx], "choice should return elements[choice_idx(p)] for same RNG state"


def run_all():
    tests = [
        test_choice_idx_basic,
        test_choice_idx_deterministic,
        test_choice_basic_strings,
        test_choice_basic_tuples,
        test_choice_preserves_identity_for_objects,
        test_choice_length_mismatch,
        test_choice_idx_bad_sum,
        test_choice_idx_negative_prob,
        test_choice_elements_not_sequence,
        test_choice_matches_choice_idx,
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