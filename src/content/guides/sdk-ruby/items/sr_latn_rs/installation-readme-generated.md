Dodajte ovu liniju u Gemfile vaše aplikacije:

```ruby
gem 'fastcomments'
```

Zatim izvršite:

```bash
bundle install
```

Ili je instalirajte sami kao:

```bash
gem install fastcomments
```

### Sadržaj biblioteke

Ova biblioteka sadrži generisani API klijent i SSO alate koji olakšavaju rad sa API-jem.

- [Dokumentacija API klijent biblioteke](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Javni vs zaštićeni API-ji

Za API klijent postoje tri klase: `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtevaju vaš API ključ, a `PublicApi` sadrži API pozive
koji se mogu izvršavati direktno iz pregledača/mobilnog uređaja itd. bez autentifikacije. `ModerationApi` sadrži metode koje pokreću moderatorsku kontrolnu tablu.

`ModerationApi` pruža opsežan skup API-ja za moderaciju u realnom vremenu. Svaka metoda `ModerationApi` prihvata `sso` parametar i može se autentifikovati putem SSO-a ili FastComments.com sesijskog kolačića.