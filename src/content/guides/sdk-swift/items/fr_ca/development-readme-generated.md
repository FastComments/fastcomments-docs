### Régénération du client

Pour régénérer le client API à partir de la dernière spécification OpenAPI :

1. Assurez-vous que le serveur FastComments est en cours d'exécution localement à `http://localhost:3001`
2. Exécutez le script de mise à jour :

```bash
./update.sh
```

Cela va :
- Télécharger la dernière spécification OpenAPI
- Générer le code client Swift (avec la documentation de l'API dans client/docs)
- Construire le paquet pour vérifier que tout fonctionne