### Exécution des tests

```bash
# Configurer les variables d'environnement
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"

# Exécuter les tests
pytest tests/
```

### Régénération du client

Pour régénérer le client API à partir de la dernière spécification OpenAPI :

```bash
./update.sh
```

Cela va :
1. Télécharger la dernière spécification OpenAPI depuis un serveur FastComments en cours d'exécution (ou utiliser le openapi.yaml local)
2. Générer le code client Python
3. Aplatir la structure des répertoires
4. Nettoyer les fichiers de configuration redondants