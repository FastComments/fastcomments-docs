### Instalirajte sa GitHub-a

Instalirajte direktno sa oznakom izdanja (preporučeno, potpuno reprodukovano):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Zakačite oznaku umesto grane kako bi izgradnje bile determinističke. Isti oblik radi u `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Svako označeno [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) takođe ima priloženu izgrađenu wheel datoteku ako radije instalirate binarni artefakt direktno.

### Sadržaj biblioteke

Ova biblioteka sadrži dva modula: generisanog API klijenta i osnovnu Python biblioteku koja sadrži ručno napisane alate za olakšavanje rada sa API-jem, uključujući SSO podršku.

- [Dokumentacija API klijentske biblioteke](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Dokumentacija osnovne biblioteke, uključujući SSO primere](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Javni vs zaštićeni API-ji

Za API klijenta postoje tri klase, `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtevaju vaš API ključ, a `PublicApi` sadrži metode koje se mogu pozvati direktno iz pregledača/mobilnog uređaja/itd. bez autentifikacije. `ModerationApi` pruža opsežan skup live i brzih moderacijskih API-ja. Svaka metoda `ModerationApi` prihvata `sso` parametar i može se autentifikovati putem SSO ili FastComments.com sesijskog kolačića.