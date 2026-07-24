### Installeren vanaf GitHub

Installeer direct vanaf een release‑tag (aanbevolen, volledig reproduceerbaar):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Pin de tag in plaats van een branch zodat builds deterministisch zijn. Dezelfde vorm werkt in `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Elke getagde [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) heeft ook een gebouwde wheel bijgevoegd als je de voorkeur geeft aan het direct installeren van een binair artefact.

### Bibliotheekinhoud

Deze bibliotheek bevat twee modules: de gegenereerde API‑client en de core Python‑bibliotheek die handgeschreven hulpprogramma’s bevat om het werken met de API gemakkelijker te maken, inclusief SSO‑ondersteuning.

- [API‑clientbibliotheekdocumentatie](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Core‑bibliotheekdocumentatie, inclusief SSO‑voorbeelden](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Openbare vs Beveiligde API’s

Voor de API‑client zijn er drie klassen, `DefaultApi`, `PublicApi` en `ModerationApi`. De `DefaultApi` bevat methoden die je API‑sleutel vereisen, en `PublicApi` bevat methoden die direct vanuit een browser/mobiel apparaat/etc. kunnen worden aangeroepen zonder authenticatie. De `ModerationApi` biedt een uitgebreide reeks live‑ en snelle moderatie‑API’s. Elke `ModerationApi`‑methode accepteert een `sso`‑parameter en kan authenticeren via SSO of een FastComments.com‑sessie‑cookie.