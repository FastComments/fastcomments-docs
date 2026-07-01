### Install from GitHub

Instalirajte direktno sa oznake izdanja (preporučeno, potpuno reproduktivno):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Zakačite oznaku umjesto grane kako bi izgradnje bile determinističke. Isti oblik radi u `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Svako označeno [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) također ima priložen izgrađeni wheel ako radije instalirate binarni artefakt direktno.

### Library Contents

Ova biblioteka sadrži dva modula: generirani API klijent i jezgru Python biblioteke koja sadrži ručno napisane alate za olakšavanje rada s API‑jem, uključujući SSO podršku.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Public vs Secured APIs

Za API klijent postoje tri klase, `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtijevaju vaš API ključ, a `PublicApi` sadrži metode koje se mogu pozvati direktno iz pregledača/mobilnog uređaja itd. bez autentifikacije. `ModerationApi` pruža opsežan skup API‑ja za brzu i trenutnu moderaciju. Svaka metoda `ModerationApi` prihvata `sso` parametar i može se autentifikovati putem SSO‑a ili FastComments.com sesijske kolačiće.