### Installer fra GitHub

Installer direkte fra et udgivelses‑tag (anbefalet, fuldt reproducerbart):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Fastgør tagget i stedet for en gren, så bygninger er deterministiske. Den samme form virker i `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Hver tagget [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) har også et bygget wheel vedhæftet, hvis du foretrækker at installere en binær artefakt direkte.

### Biblioteksindhold

Dette bibliotek indeholder to moduler: den genererede API‑klient og kjerne‑Python‑biblioteket, som indeholder håndskrevne værktøjer for at gøre arbejdet med API'en lettere, herunder SSO‑understøttelse.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Offentlige vs Sikrede API'er

For API‑klienten er der tre klasser, `DefaultApi`, `PublicApi` og `ModerationApi`. `DefaultApi` indeholder metoder, der kræver din API‑nøgle, og `PublicApi` indeholder metoder, der kan udføres direkte fra en browser/mobil enhed osv. uden autentifikation. `ModerationApi` leverer en omfattende række af live og hurtige moderations‑API'er. Hver `ModerationApi`‑metode accepterer en `sss`‑parameter og kan autentificere via SSO eller en FastComments.com‑session cookie.