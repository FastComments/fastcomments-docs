---
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

Ta knjižnica vsebuje generiran odjemalec API in SSO orodja, ki olajšajo delo z API-jem.

- [Dokumentacija knjižnice API odjemalca](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Javni in zaščiteni API-ji

Za odjemalca API sta na voljo dva razreda, `DefaultAPI` in `PublicAPI`. `DefaultAPI` vsebuje metode, ki zahtevajo vaš API ključ, medtem ko `PublicAPI` vsebuje klice API-ja, ki jih je mogoče izvesti neposredno iz brskalnika/mobilne naprave/itd. brez preverjanja pristnosti.
---