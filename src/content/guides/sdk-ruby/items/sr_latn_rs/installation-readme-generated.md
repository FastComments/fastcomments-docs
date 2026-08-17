Add this line to your application's Gemfile:

```ruby
gem 'fastcomments'
```

And then execute:

```bash
bundle install
```

Or install it yourself as:

```bash
gem install fastcomments
```

### Library Contents

Ova biblioteka sadrži generisanog API klijenta i SSO alate koji olakšavaju rad sa API‑jem.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Public vs Secured APIs

Za API klijenta postoje tri klase, `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtevaju vaš API ključ, a `PublicApi` sadrži API pozive koji se mogu izvršiti direktno iz pregledača/mobilnog uređaja itd. bez autentifikacije. `ModerationApi` sadrži metode koje napajaju moderatorsku kontrolnu tablu.

`ModerationApi` pruža opsežan skup API‑ja za moderaciju u realnom vremenu i brzu moderaciju. Svaka metoda `ModerationApi` prihvata `sso` parametar i može se autentifikovati putem SSO‑a ili FastComments.com sesijskog kolačića.