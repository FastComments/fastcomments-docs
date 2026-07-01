### Namestitev odvisnosti

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Gradnja iz izvorne kode

```bash
mkdir build
cd build
cmake ..
make
```

### Namestitev

```bash
sudo make install
```

### Vsebina knjižnice

Ta knjižnica vsebuje ustvarjen odjemalec API-ja in orodja SSO, ki olajšajo delo z API-jem.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Javni in zavarovani API-ji

Za odjemalca API obstajajo trije razredi, `DefaultApi`, `PublicApi` in `ModerationApi`. `DefaultApi` vsebuje metode, ki zahtevajo vaš API ključ, `PublicApi` pa metode, ki jih je mogoče izvajati neposredno iz brskalnika/naprave mobilnega telefona/itd brez avtentikacije. `ModerationApi` nudi obsežen nabor živih in hitrih moderacijskih API-jev. Vsaka metoda `ModerationApi` sprejme parameter `sso` in se lahko avtenticira prek SSO ali piškotka seje FastComments.com.