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

Ova biblioteka sadrži generirani API klijent i SSO alate koji olakšavaju rad s API‑jem.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Public vs Secured APIs

Za API klijent postoje tri klase, `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtijevaju vaš API ključ, a `PublicApi` sadrži metode koje se mogu izvesti direktno iz preglednika/mobilnog uređaja/itd. bez autentifikacije. `ModerationApi` pruža opsežan skup live i brzih API‑ja za moderaciju. Svaka metoda `ModerationApi` prihvata parametar `sso` i može se autentifikovati putem SSO‑a ili sesijskog kolačića FastComments.com.