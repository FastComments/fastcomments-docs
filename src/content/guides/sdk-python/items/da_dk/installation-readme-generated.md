### PyPI

```bash
pip install fastcomments
```

### Bibliotekets indhold

Dette bibliotek indeholder to moduler: den genererede API-klient og kerne-Python-biblioteket, som indeholder håndskrevne hjælpefunktioner, der gør arbejdet med API'en nemmere, inklusive SSO-understøttelse.

- [Dokumentation for API-klientbiblioteket](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Kernebibliotekets dokumentation, inklusive SSO-eksempler](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Offentlige vs Sikrede API'er

For API-klienten findes der to klasser, `DefaultApi` og `PublicApi`. `DefaultApi` indeholder metoder, der kræver din API-nøgle, og `PublicApi` indeholder API-opkald, som kan foretages direkte fra en browser/mobil enhed mv. uden autentificering.