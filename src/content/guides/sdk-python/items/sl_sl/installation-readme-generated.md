### PyPI

```bash
pip install fastcomments
```

### Vsebina knjižnice

Ta knjižnica vsebuje dva modula: samodejno ustvarjen odjemalec API in jedrna Python knjižnica, ki vsebuje ročno napisane pripomočke za lažje delo z API-jem, vključno s podporo SSO.

- [Dokumentacija API odjemalca](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Dokumentacija jedrne knjižnice, vključno s primeri SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Javne in zavarovane API-je

Za API odjemalca so na voljo trije razredi, `DefaultApi`, `PublicApi` in `ModerationApi`. `DefaultApi` vsebuje metode, ki zahtevajo vaš API ključ, medtem ko `PublicApi` vsebuje metode, ki jih je mogoče klicati neposredno iz brskalnika/mobilne naprave/itd. brez overjanja. `ModerationApi` poganja nadzorno ploščo moderatorja in vsebuje metode za moderiranje komentarjev (seznam, štetje, iskanje, dnevni zapisi, izvoz), ukrepe moderiranja (odstrani/obnovi, označi, nastavi status pregleda/spama/odobritve, glasovi, ponovno odpri/zaključi nit), prepovedi (prepoved komentiranja, razveljavitev, povzetki pred prepovedjo, stanje/nastavitve prepovedi, štetje prepovedanih uporabnikov) ter značke in zaupanje (podeli/odstrani značko, ročne značke, pridobi/nastavi faktor zaupanja, uporabniški notranji profil). Vsaka metoda `ModerationApi` sprejme parameter `sso`, tako da jo je mogoče klicati v imenu moderatorja, avtenticiranega preko SSO.