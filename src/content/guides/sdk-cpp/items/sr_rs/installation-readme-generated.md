### Instalirajte zavisnosti

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

### Instaliranje

```bash
sudo make install
```

### Sadržaj biblioteke

Ova biblioteka sadrži generisani API klijent i SSO alate koji olakšavaju rad sa API-jem.

- [Dokumentacija API klijentske biblioteke](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Javni vs zaštićeni API-ji

Za API klijent postoje tri klase, `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtevaju vaš API ključ, a `PublicApi` sadrži
metode koje se mogu pozvati direktno iz pregledača/mobilnog uređaja/i sl. bez autentifikacije. `ModerationApi` pruža opsežan skup API-ja za moderaciju u realnom vremenu i brzu moderaciju. Svaka metoda `ModerationApi` prihvata `sso` parametar i može se autentifikovati putem SSO ili sesijskog kolačića sa FastComments.com.