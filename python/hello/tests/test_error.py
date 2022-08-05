import pytest
import hello


def test_throw_error():
    with pytest.raises(BaseException, match="error from Rust!"):
        hello.error.throw_error()
