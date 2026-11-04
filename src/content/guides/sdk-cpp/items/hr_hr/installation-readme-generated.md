### Instalacija ovisnosti

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Gradnja iz izvornog koda

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

Ova biblioteka sadrži generirani API klijent i SSO uslužne programe koji olakšavaju rad s API-jem.

- [Dokumentacija API klijentske biblioteke](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Javni naspram osiguranih API-ja

Za API klijent postoje tri klase: `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtijevaju vaš API ključ, a `PublicApi` sadrži metode koje se mogu izvršavati izravno iz preglednika/mobilnog uređaja itd. bez autentifikacije. `ModerationApi` pruža opsežan skup API-ja za brzu moderaciju u stvarnom vremenu. Svaka metoda `ModerationApi` prihvaća `sso` parametar i može se autentificirati putem SSO-a ili kolačića sesije FastComments.com.