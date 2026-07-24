### Instalirajte sa GitHub-a

Instalirajte direktno sa oznakom izdanja (preporučeno, potpuno reprodukovano):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Zakačite oznaku umjesto grane kako bi izgradnje bile determinističke. Isti oblik radi u `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Svaka označena [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) takođe ima priložen prekompajlirani paket (wheel) ako više volite da instalirate binarni artefakt direktno.

### Sadržaj biblioteke

Ova biblioteka sadrži dva modula: generisani API klijent i osnovnu Python biblioteku koja sadrži ručno napisane alate za olakšavanje rada sa API‑jem, uključujući SSO podršku.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Javni vs Zaštićeni API‑ji

Za API klijent postoje tri klase, `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtijevaju vaš API ključ, a `PublicApi` sadrži metode koje se mogu pozvati direktno iz preglednika/mobilnog uređaja itd. bez autentifikacije. `ModerationApi` pruža opsežan skup API‑ja za brzu i živu moderaciju. Svaka metoda `ModerationApi` prihvata `sso` parametar i može se autentifikovati putem SSO‑a ili FastComments.com sesijskog kolačića.