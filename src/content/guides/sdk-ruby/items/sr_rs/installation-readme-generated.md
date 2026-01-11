Dodajte ovu liniju u Gemfile vaše aplikacije:

```ruby
gem 'fastcomments'
```

A zatim izvršite:

```bash
bundle install
```

Ili ga instalirajte sami kao:

```bash
gem install fastcomments
```

### Sadržaj biblioteke

Ova biblioteka sadrži generisan API klijent i SSO pomoćne alatke koje olakšavaju rad sa API-jem.

- [Dokumentacija API klijenta biblioteke](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Javni i zaštićeni API-ji

Za API klijenta postoje dve klase, `DefaultApi` i `PublicApi`. `DefaultApi` sadrži metode koje zahtevaju vaš API ključ, a `PublicApi` sadrži API pozive
koji se mogu pozivati direktno iz pregledača/mobilnog uređaja/itd. bez autentifikacije.