### Instalirajte zavisnosti

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Kompiliranje iz izvornog koda

```bash
mkdir build
cd build
cmake ..
make
```

### Instaliranje

```bash
sudo make install
```

### Sadržaj biblioteke

Ova biblioteka sadrži generisan API klijent i SSO alate koji olakšavaju rad sa API-jem.

- [Dokumentacija API klijentske biblioteke](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Javni i zaštićeni API-ji

Za API klijenta postoje dvije klase, `DefaultAPI` i `PublicAPI`. `DefaultAPI` sadrži metode koje zahtijevaju vaš API key, a `PublicAPI` sadrži API pozive koji se mogu direktno izvršavati iz preglednika/mobilnog uređaja/itd. bez autentifikacije.