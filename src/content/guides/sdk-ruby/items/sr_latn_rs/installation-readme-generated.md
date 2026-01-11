---
Dodajte ovu liniju u Gemfile vaše aplikacije:

```ruby
gem 'fastcomments'
```

A zatim izvršite:

```bash
bundle install
```

Ili ga instalirajte ručno kao:

```bash
gem install fastcomments
```

### Sadržaj biblioteke

Ova biblioteka sadrži generisan API klijent i SSO alate koji olakšavaju rad sa API-jem.

- [Dokumentacija API klijentske biblioteke](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Javne vs Zaštićene API-je

Za API klijenta, postoje dve klase, `DefaultApi` i `PublicApi`. `DefaultApi` sadrži metode koje zahtevaju vaš API ključ, dok `PublicApi` sadrži API pozive koji se mogu izvršiti direktno iz pregledača/mobilnog uređaja/itd. bez autentifikacije.
---