---
### De client opnieuw genereren

Om de API-client te regenereren vanaf de nieuwste OpenAPI-specificatie:

1. Zorg ervoor dat je de FastComments-server lokaal hebt draaien op `http://localhost:3001`
2. Voer het update-script uit:

```bash
./update.sh
```

Dit zal:
- Download de nieuwste OpenAPI-specificatie
- Genereer de Swift-clientcode (met API-documentatie in client/docs)
- Bouw het pakket om te verifiëren dat alles werkt
---