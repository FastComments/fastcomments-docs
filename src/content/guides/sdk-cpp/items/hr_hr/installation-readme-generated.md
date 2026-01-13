### Instalirajte ovisnosti

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

Ova biblioteka sadrži generirani API klijent i SSO pomoćne alate koji olakšavaju rad s API-jem.

- [Dokumentacija API klijentske biblioteke](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Javni vs Zaštićeni API-ji

Za API klijenta postoje dvije klase, `DefaultAPI` i `PublicAPI`. Klasa `DefaultAPI` sadrži metode koje zahtijevaju vaš API ključ, a `PublicAPI` sadrži pozive API-ja
koji se mogu izvršiti izravno iz preglednika/na mobilnom uređaju/itd. bez autentikacije.