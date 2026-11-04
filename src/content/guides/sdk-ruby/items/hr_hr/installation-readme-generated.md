Dodajte ovu liniju u Gemfile svoje aplikacije:

```ruby
gem 'fastcomments'
```

Zatim izvršite:

```bash
bundle install
```

Ili je instalirajte sami pomoću:

```bash
gem install fastcomments
```

### Sadržaj biblioteke

Ova biblioteka sadrži generirani API klijent i SSO uslužne programe koji olakšavaju rad s API-jem.

- [Dokumentacija API klijentske biblioteke](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Javni naspram osiguranih API-ja

Za API klijent postoje tri klase: `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtijevaju vaš API ključ, a `PublicApi` sadrži API pozive
koji se mogu izvršavati izravno iz preglednika/mobilnog uređaja itd. bez autentifikacije. `ModerationApi` sadrži metode koje pokreću moderatorsku ploču.

`ModerationApi` pruža opsežan skup API-ja za moderiranje u stvarnom vremenu. Svaka metoda `ModerationApi` prihvaća `sso` parametar i može se autentificirati putem SSO-a ili kolačića sesije FastComments.com.