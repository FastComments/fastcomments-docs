### Instaliranje zavisnosti

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Izgradnja iz izvornog koda

```bash
mkdir build
cd build
cmake ..
make
```

### Instalacija

```bash
sudo make install
```

### Sadržaj biblioteke

Ova biblioteka sadrži generisan API klijent i SSO alatke koje olakšavaju rad sa API-jem.

- [Dokumentacija biblioteke API klijenta](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Javni naspram zaštićenih API-ja

Za API klijenta postoje dve klase, `DefaultAPI` i `PublicAPI`. `DefaultAPI` sadrži metode koje zahtevaju vaš API ključ, a `PublicAPI` sadrži pozive API-ja koji se mogu izvršavati direktno iz pregledača/mobilnog uređaja/itd. bez autentifikacije.