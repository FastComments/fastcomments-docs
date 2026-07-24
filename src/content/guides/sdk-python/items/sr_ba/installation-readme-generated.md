### Instalacija sa GitHub-a

Instalirajte direktno sa oznake izdanja (preporučeno, potpuno reprodukovano):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Fiksirajte oznaku umjesto grane kako bi izgradnje bile determinističke. Isti oblik radi u `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Svaka označena [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) takođe ima priloženi wheel paket ako radije želite direktno instalirati binarni artefakt.

### Sadržaj biblioteke

Ova biblioteka sadrži dva modula: generisani API klijent i osnovnu Python biblioteku koja sadrži ručno napisane alate za olakšavanje rada sa API‑jem, uključujući SSO podršku.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Javni vs zaštićeni API‑ji

Za API klijent postoje tri klase, `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtijevaju vaš API ključ, a `PublicApi` sadrži metode koje se mogu pozvati direktno iz preglednika/mobilnog uređaja itd. bez autentifikacije. `ModerationApi` pruža opsežan skup live i brzih moderacijskih API‑ja. Svaka metoda `ModerationApi` prihvata `sso` parametar i može se autentifikovati putem SSO‑a ili FastComments.com sesijskog kolačića.