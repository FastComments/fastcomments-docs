---
Dodajte to vrstico v Gemfile vaše aplikacije:

```ruby
gem 'fastcomments'
```

Nato izvedite:

```bash
bundle install
```

Ali ga namestite sami z:

```bash
gem install fastcomments
```

### Vsebina knjižnice

Ta knjižnica vsebuje generiran odjemalec API in SSO pripomočke, ki olajšajo delo z API-jem.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Javni in zavarovani API-ji

Za odjemalca API so na voljo tri razrede, `DefaultApi`, `PublicApi` in `ModerationApi`. `DefaultApi` vsebuje metode, ki zahtevajo vaš API ključ, `PublicApi` pa vsebuje klice API,
ki jih je mogoče izvesti neposredno iz brskalnika/mobilne naprave/itd. brez overjanja. `ModerationApi` vsebuje metode, ki poganjajo nadzorno ploščo moderatorja.

`ModerationApi` zajema moderiranje komentarjev (seznam, štetje, iskanje, dnevniki, izvoz), ukrepe moderiranja (odstrani/obnovi, prijavi, nastavi status pregleda/spama/odobritve, glasovi, ponovno odpri/zapri nit),
prepovedi (prepoved na podlagi komentarja, razveljavitev, povzetki pred prepovedjo, status/nastavitve prepovedi, število prepovedanih uporabnikov), in značke & zaupanje (podeli/odstrani značko, ročne značke, pridobi/nastavi dejavnik zaupanja, notranji profil uporabnika).
Vsaka metoda `ModerationApi` sprejme parameter `sso`, tako da je lahko zahteva izvedena v imenu moderatorja, avtenticiranega preko SSO.
---