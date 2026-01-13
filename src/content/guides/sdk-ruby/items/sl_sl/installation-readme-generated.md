Dodajte to vrstico v Gemfile vaše aplikacije:

```ruby
gem 'fastcomments'
```

Nato zaženite:

```bash
bundle install
```

Ali ga namestite sami z ukazom:

```bash
gem install fastcomments
```

### Vsebina knjižnice

Ta knjižnica vsebuje generiranega odjemalca API in pripomočke SSO, ki olajšajo delo z API-jem.

- [Dokumentacija knjižnice API odjemalca](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Javne in zaščitene API-ji

Za odjemalca API sta na voljo dva razreda, `DefaultApi` in `PublicApi`. `DefaultApi` vsebuje metode, ki zahtevajo vaš API ključ, `PublicApi` pa vsebuje klice API, ki jih je mogoče izvesti neposredno iz brskalnika/mobilne naprave/itd. brez avtentikacije.