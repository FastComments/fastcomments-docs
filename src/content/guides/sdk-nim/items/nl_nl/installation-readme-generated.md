### Nimble gebruiken

```bash
nimble install fastcomments
```

### Bouwen vanuit de bron

```bash
nimble build
```

### Inhoud van de bibliotheek

Deze bibliotheek bevat de gegenereerde API-client en de SSO-hulpprogramma's om het werken met de API te vergemakkelijken.

- [Documentatie van de API-clientbibliotheek](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Openbare vs beveiligde API's

Voor de API-client zijn er twee API-modules, `api_default` en `api_public`. De `api_default` bevat methoden die uw API-sleutel vereisen, en `api_public` bevat API-aanroepen
die rechtstreeks vanuit een browser/mobiel apparaat/enz. kunnen worden gedaan zonder authenticatie.