### Install from GitHub

Instalirajte direktno sa oznakom izdanja (preporučeno, potpuno reprodukovano):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Zakačite oznaku umesto grane kako bi izgradnje bile determinističke. Isti format radi u `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Svaka označena [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) takođe ima priloženi izgrađeni wheel ako više volite da instalirate binarni artefakt direktno.

### Library Contents

Ova biblioteka sadrži dva modula: generisani API klijent i osnovnu Python biblioteku koja sadrži ručno napisane alate za olakšavanje rada sa API‑jem, uključujući SSO podršku.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Public vs Secured APIs

Za API klijent postoje tri klase, `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtevaju vaš API ključ, a `PublicApi` sadrži metode koje se mogu pozvati direktno iz pregledača/mobilnog uređaja/itd. bez autentifikacije. `ModerationApi` pruža opsežan skup live i brzih API‑ja za moderaciju. Svaka metoda `ModerationApi` prihvata parametar `sso` i može se autentifikovati putem SSO ili FastComments.com sesijskog kolačića.