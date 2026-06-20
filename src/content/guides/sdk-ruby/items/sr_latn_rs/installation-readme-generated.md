Dodajte ovu liniju u Gemfile vaše aplikacije:

```ruby
gem 'fastcomments'
```

A zatim izvršite:

```bash
bundle install
```

Ili instalirajte ručno kao:

```bash
gem install fastcomments
```

### Sadržaj biblioteke

Ova biblioteka sadrži generisan API klijent i SSO utilitete koji olakšavaju rad sa API-jem.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Javni i zaštićeni API-ji

Za API klijenta postoje tri klase, `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtevaju vaš API ključ, a `PublicApi` sadrži pozive API-ja koji se mogu izvršiti direktno iz pregledača/mobilnog uređaja/itd. bez autentifikacije. `ModerationApi` sadrži metode koje pokreću kontrolnu tablu moderatora.

`ModerationApi` obuhvata moderiranje komentara (lista, broj, pretraga, zapisi, izvoz), moderacione akcije (ukloni/vrati, prijavi, postavi status na pregled/špamski/odobreno, glasovi, ponovo otvori/zatvori nit), zabrane (zabrana iz komentara, poništi, pregledi pre zabrane, status/preferencije zabrane, brojevi zabranjenih korisnika) i bedževe i poverenje (dodeli/ukloni bedž, ručni bedževi, dohvati/postavi faktor poverenja, interni profil korisnika). Svaka metoda `ModerationApi` prima parametar `sso` tako da zahtev može biti napravljen u ime moderatora koji je autentifikovan putem SSO.