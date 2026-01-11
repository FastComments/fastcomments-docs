Dodajte ovaj redak u Gemfile vaše aplikacije:

```ruby
gem 'fastcomments'
```

Zatim izvršite:

```bash
bundle install
```

Ili ga sami instalirajte ovako:

```bash
gem install fastcomments
```

### Sadržaj biblioteke

Ova biblioteka sadrži generirani API klijent i SSO alate koji olakšavaju rad s API-jem.

- [Dokumentacija knjižnice API klijenta](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Javni vs Zaštićeni API-ji

Za API klijenta postoje dvije klase, `DefaultApi` i `PublicApi`. `DefaultApi` sadrži metode koje zahtijevaju vaš API ključ, a `PublicApi` sadrži API pozive koji se mogu izvršiti izravno iz preglednika/mobilnog uređaja/itd. bez autentikacije.
---