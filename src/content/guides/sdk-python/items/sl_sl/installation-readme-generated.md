### PyPI

```bash
pip install fastcomments
```

### Vsebina knjižnice

Ta knjižnica vsebuje dva modula: generirani odjemalec API in jedrna Python knjižnica, ki vsebuje ročno napisane pripomočke za lažje delo z API-jem, vključno s podporo za SSO.

- [Dokumentacija odjemalske knjižnice API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Dokumentacija jedrne knjižnice, vključno s primeri SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Javni in zavarovani API-ji

Za API odjemalca sta na voljo dve razreda, `DefaultApi` in `PublicApi`. `DefaultApi` vsebuje metode, ki zahtevajo vaš API ključ, medtem ko `PublicApi` vsebuje klice API, ki jih je mogoče izvesti neposredno iz brskalnika, mobilne naprave ipd. brez overjanja.