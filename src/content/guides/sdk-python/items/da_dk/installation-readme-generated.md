### Installér fra GitHub

Installér direkte fra et udgivelsestag (anbefalet, fuldt reproducerbart):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Fastgør tagget i stedet for en gren, så builds er deterministiske. Den samme form fungerer i `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Hver tagget [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) har også et bygget wheel vedhæftet, hvis du foretrækker at installere en binær artefakt direkte.

### Biblioteksindhold

Dette bibliotek indeholder to moduler: den genererede API-klient og kerne‑Python‑biblioteket, som indeholder håndskrevne værktøjer for at gøre arbejdet med API’en lettere, inklusive SSO‑understøttelse.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Offentlige vs Sikrede API’er

Denne API‑klient har tre klasser, `DefaultApi`, `PublicApi` og `ModerationApi`. `DefaultApi` indeholder metoder, der kræver din API‑nøgle, og `PublicApi` indeholder metoder, som kan kaldes direkte fra en browser/mobil enhed osv. uden autentificering. `ModerationApi` leverer en omfattende suite af live‑ og hurtige moderations‑API’er. Hver `ModerationApi`‑metode accepterer en `sso`‑parameter og kan autentificere via SSO eller en FastComments.com‑sessions‑cookie.