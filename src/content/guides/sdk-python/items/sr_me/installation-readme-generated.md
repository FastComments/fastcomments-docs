### Instaliraj sa GitHub-a

Instalirajte direktno sa oznake izdanja (preporučeno, potpuno reproducibilno):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Zakačite oznaku umjesto grane kako bi gradnje bile determinističke. Isti oblik radi u `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Svaki označeni [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) takođe ima priložen wheel fajl ako radije instalirate binarni artefakt direktno.

### Sadržaj biblioteke

Ova biblioteka sadrži dva modula: generisani API klijent i osnovnu Python biblioteku koja sadrži ručno napisane alate kako bi rad s API‑jem bio jednostavniji, uključujući podršku za SSO.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Javni vs Zaštićeni API‑ji

Za API klijent postoje tri klase, `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtijevaju vaš API ključ, a `PublicApi` sadrži metode koje se mogu pozivati direktno iz pregledača/mobilnog uređaja/etc. bez autentifikacije. `ModerationApi` pruža opsežan skup live i brzih API‑ja za moderaciju. Svaka metoda `ModerationApi` prima parametar `sso` i može se autentifikovati putem SSO‑a ili sesijskog kolačića FastComments.com.