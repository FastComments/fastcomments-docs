### Install Dependencies

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Building from Source

```bash
mkdir build
cd build
cmake ..
make
```

### Installing

```bash
sudo make install
```

### Library Contents

This library contains the generated API client and the SSO utilities to make working with the API easier.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Public vs Secured APIs

For the API client, there are two classes, `DefaultAPI` and `PublicAPI`. The `DefaultAPI` contains methods that require your API key, and `PublicAPI` contains api calls
that can be made directly from a browser/mobile device/etc without authentication.