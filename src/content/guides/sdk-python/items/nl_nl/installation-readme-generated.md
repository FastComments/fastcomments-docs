### PyPI

```bash
pip install fastcomments
```

### Bibliotheekinhoud

Deze bibliotheek bevat twee modules: de gegenereerde API-client en de core Python-bibliotheek die handgeschreven hulpprogramma's bevat om het werken met de API gemakkelijker te maken, inclusief SSO-ondersteuning.

- [Documentatie API-clientbibliotheek](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core-bibliotheekdocumentatie, inclusief SSO-voorbeelden](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Publieke vs Beveiligde API's

Voor de API-client zijn er twee klassen, `DefaultApi` en `PublicApi`. De `DefaultApi` bevat methoden die uw API-sleutel vereisen, en `PublicApi` bevat API-aanroepen die rechtstreeks vanuit een browser/mobile device/etc zonder authenticatie kunnen worden uitgevoerd.