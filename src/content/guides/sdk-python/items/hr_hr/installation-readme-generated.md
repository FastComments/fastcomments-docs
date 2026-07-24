### Instaliraj s GitHub-a

Instaliraj izravno iz oznake izdanja (preporučeno, potpuno reproducibilno):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Zakači oznaku umjesto grane kako bi izgradnje bile determinističke. Isti oblik radi u `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Svako označeno [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) također ima priloženi izgrađeni wheel ako radije izravno instaliraš binarni artefakt.

### Sadržaj biblioteke

Ova biblioteka sadrži dva modula: generirani API klijent i osnovnu Python biblioteku koja sadrži ručno napisane alate za olakšavanje rada s API-jem, uključujući SSO podršku.

- [Dokumentacija API klijentske biblioteke](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Dokumentacija osnovne biblioteke, uključujući SSO primjere](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Javni vs zaštićeni API-ji

Za API klijent postoje tri klase, `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtijevaju vaš API ključ, a `PublicApi` sadrži metode koje se mogu izravno pozvati iz preglednika/mobilnog uređaja/itd. bez autentifikacije. `ModerationApi` pruža opsežan skup live i brzih moderacijskih API-ja. Svaka metoda `ModerationApi` prihvaća parametar `sso` i može se autentificirati putem SSO-a ili FastComments.com sesijskog kolačića.