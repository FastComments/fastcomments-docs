---
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

Ova biblioteka sadrži generisan API klijent i SSO pomoćne alatke da olakša rad sa API-jem.

- [Dokumentacija API klijentske biblioteke](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Javne naspram zaštićenih API-ja

Za API klijenta postoje dvije klase, `DefaultApi` i `PublicApi`. `DefaultApi` sadrži metode koje zahtijevaju vaš API ključ, a `PublicApi` sadrži pozive API-ja
koji se mogu izvršiti direktno iz pregledača/mobilnog uređaja/itd. bez autentifikacije.
---