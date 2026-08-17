Dodajte to vrstico v datoteko Gemfile vaše aplikacije:

```ruby
gem 'fastcomments'
```

Nato izvedite:

```bash
bundle install
```

Ali ga namestite sami kot:

```bash
gem install fastcomments
```

### Vsebina knjižnice

Ta knjižnica vsebuje ustvarjenega API odjemalca in pripomočke SSO, ki olajšajo delo z API-jem.

- [Dokumentacija knjižnice API odjemalca](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Javni vs Zavarovani API-ji

Za API odjemalca obstajajo trije razredi, `DefaultApi`, `PublicApi` in `ModerationApi`. `DefaultApi` vsebuje metode, ki zahtevajo vaš API ključ, `PublicApi` vsebuje klice API, ki jih je mogoče izvesti neposredno iz brskalnika/mobilne naprave/ipd. brez avtentikacije. `ModerationApi` vsebuje metode, ki poganjajo nadzorno ploščo moderatorja.

`ModerationApi` ponuja obsežen nabor živo in hitro moderacijskih API-jev. Vsaka metoda `ModerationApi` sprejme parameter `sso` in se lahko avtenticira prek SSO ali piškotka seje FastComments.com.