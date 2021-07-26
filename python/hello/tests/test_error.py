import pytest
import hello

def test_raise_error():
    with pytest.raises(BaseException, match="error raised from Rust!"):
        hello.error.raise_error()
