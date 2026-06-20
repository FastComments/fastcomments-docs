### Namestitev odvisnosti

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Sestavljanje iz izvorne kode

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

Ta knjižnica vsebuje generiran API odjemalec in SSO pripomočke, da je delo z API-jem lažje.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Javni in zaščiteni API-ji

Za API odjemalca obstajajo trije razredi, `DefaultApi`, `PublicApi`, in `ModerationApi`. Razred `DefaultApi` vsebuje metode, ki zahtevajo vaš API ključ, razred `PublicApi` pa vsebuje
metode, ki jih je mogoče klicati neposredno iz brskalnika/mobilne naprave/itd. brez avtentikacije. Razred `ModerationApi` vsebuje metode, ki poganjajo nadzorno ploščo moderatorja - izpis,
števanje, iskanje, izvoz in pridobivanje dnevnikov za komentarje, moderacijske akcije (odstrani/obnovi, prijavi, nastavi status pregleda/spama/odobritve, prilagodi glasove, ponovno odpri/zaključi niti),
prepovedi (prepoved iz komentarja, razveljavitev prepovedi, povzetki pred prepovedjo, stanje prepovedi in nastavitve, število prepovedanih uporabnikov), in značke & zaupanje (podeli/odstrani značke, ročne značke, pridobi/nastavi faktor zaupanja
faktor, notranji uporabniški profil). Vsaka metoda `ModerationApi` sprejme parameter `sso`, zato se klic izvede v imenu moderatorja, avtenticiranega preko SSO.