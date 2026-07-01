### Instalacija sa GitHub-a

Instalirajte direktno iz taga verzije (preporučeno, potpuno reprodukovano):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Fiksirajte tag umesto grane kako bi izgradnje bile determinističke. Isti oblik radi u `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Svaki označeni [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) takođe ima priloženu izgrađenu wheel datoteku ako više volite da instalirate binarni artefakt direktno.

### Sadržaj biblioteke

Ova biblioteka sadrži dva modula: generisani API klijent i osnovnu Python biblioteku koja sadrži ručno napisane alate za olakšavanje rada sa API‑jem, uključujući SSO podršku.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Javna vs Zaštićena API‑ja

Za API klijent postoje tri klase, `DefaultApi`, `PublicApi` i `ModerationApi`. `DefaultApi` sadrži metode koje zahtevaju vaš API ključ, dok `PublicApi` sadrži metode koje se mogu pozvati direktno iz pregledača/mobilnog uređaja/itd. bez autentifikacije. `ModerationApi` pruža opsežan skup live i brzih API‑ja za moderaciju. Svaka metoda `ModerationApi` prihvata `sso` parametar i može se autentifikovati putem SSO‑a ili FastComments.com kolačića sesije.