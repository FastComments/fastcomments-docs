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

### Instalacija

```bash
sudo make install
```

### Sadržaj biblioteke

Ova biblioteka sadrži generisan API klijent i SSO utilitare koji olakšavaju rad sa API-jem.

- [Dokumentacija API klijentske biblioteke](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Javni naspram zaštićenih API-ja

Za API klijent postoje tri klase, `DefaultApi`, `PublicApi`, i `ModerationApi`. `DefaultApi` sadrži metode kojima je potreban vaš API ključ, a `PublicApi` sadrži
metode koje se mogu pozivati direktno iz pregledača/mobilnog uređaja/itd bez autentifikacije. `ModerationApi` sadrži metode koje pokreću kontrolnu tablu moderatora - listanje,
brojanje, pretragu, izvoz i preuzimanje logova za komentare, akcije moderacije (uklanjanje/vraćanje, označavanje, postavljanje statusa za pregled/spam/odobrenje, podešavanje glasova, ponovno otvaranje/zatvaranje tema),
zabrane (zabrana komentarisanja, poništavanje zabrana, sažeci pre-zabrane, status i podešavanja zabrane, broj zabranjenih korisnika), i značke i poverenje (dodela/uklanjanje znački, ručne značke, dobijanje/postavljanje faktora poverenja, unutrašnji profil korisnika). Svaka metoda iz `ModerationApi` prihvata `sso` parametar tako da se poziv izvršava u ime moderatora autentifikovanog putem SSO.