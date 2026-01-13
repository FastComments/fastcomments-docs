### Installer afhængigheder

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Kompilering fra kildekode

```bash
mkdir build
cd build
cmake ..
make
```

### Installation

```bash
sudo make install
```

### Biblioteksindhold

Dette bibliotek indeholder den genererede API-klient og SSO-værktøjerne, som gør det lettere at arbejde med API'en.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Offentlige vs Beskyttede API'er

For API-klienten er der to klasser, `DefaultAPI` og `PublicAPI`. `DefaultAPI` indeholder metoder, der kræver din API-nøgle, og `PublicAPI` indeholder API-opkald
der kan foretages direkte fra en browser/mobil enhed/osv. uden godkendelse.