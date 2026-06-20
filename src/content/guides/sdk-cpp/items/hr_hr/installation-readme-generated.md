### Instalacija ovisnosti

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Sastavljanje iz izvornog koda

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

Ova biblioteka sadrži generirani API klijent i SSO utilitete koji olakšavaju rad s API-jem.

- [Dokumentacija API klijentske biblioteke](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Javni i zaštićeni API-ji

Za API klijenta postoje tri klase, `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtijevaju vaš API ključ, a `PublicApi` sadrži
metode koje se mogu pozivati izravno iz preglednika/mobilnog uređaja/itd. bez autentifikacije. `ModerationApi` sadrži metode koje pokreću nadzornu ploču moderatora - popis,
brojanje, pretraživanje, izvoz i povlačenje zapisnika za komentare, akcije moderiranja (uklanjanje/obnavljanje, prijava, postavljanje statusa pregleda/spama/odobrenja, podešavanje glasova, ponovno otvaranje/zaključavanje niti),
zabrane (blokiranje s komentara, poništavanje zabrana, sažeci prije zabrane, status i postavke zabrane, broj zabranjenih korisnika), i značke & povjerenje (dodjela/uklanjanje znački, ručno dodijeljene značke, dohvat/postavljanje
faktora povjerenja, interni profil korisnika). Svaka metoda `ModerationApi` prihvaća parametar `sso` tako da se poziv izvršava u ime moderatora autentificiranog putem SSO-a.