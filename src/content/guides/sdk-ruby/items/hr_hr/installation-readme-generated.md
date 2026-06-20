Dodajte ovaj redak u Gemfile vaše aplikacije:

```ruby
gem 'fastcomments'
```

Zatim izvršite:

```bash
bundle install
```

Ili ga instalirajte sami pomoću:

```bash
gem install fastcomments
```

### Library Contents

Ova knjižnica sadrži generirani API klijent i SSO pomoćne alate koji olakšavaju rad s API-jem.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Public vs Secured APIs

Za API klijent postoje tri klase, `DefaultApi`, `PublicApi`, i `ModerationApi`. `DefaultApi` sadrži metode koje zahtijevaju vaš API ključ, a `PublicApi` sadrži pozive API-ja koje je moguće izvršiti izravno iz preglednika/mobilnog uređaja/itd. bez autentifikacije. `ModerationApi` sadrži metode koje pokreću nadzornu ploču moderatora.

`ModerationApi` obuhvaća moderiranje komentara (lista, broj, pretraživanje, zapisi, izvoz), radnje moderiranja (ukloni/vrati, prijavi, postavi status pregledavanja/špama/odobrenja, glasovi, ponovno otvori/zatvori nit),
zabrane (zabrana na komentaru, poništi, sažetci pred-zabrane, status/preferencije zabrane, brojevi zabranjenih korisnika) i bedževe & povjerenje (dodijeli/ukloni bedž, ručni bedževi, dobivanje/postavljanje faktora povjerenja, internI profil korisnika).
Svaka metoda `ModerationApi` prima `sso` parametar tako da se zahtjev može izvesti u ime moderatora autentificiranog putem SSO-a.