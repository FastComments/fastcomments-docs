### Tests ausführen

```bash
# Set up environment variables
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"

# Run tests
pytest tests/
```

### Client neu generieren

Um den API-Client aus der neuesten OpenAPI-Spezifikation neu zu generieren:

```bash
./update.sh
```

Dies wird:
1. Die aktuelle OpenAPI-Spezifikation von einem laufenden FastComments-Server herunterladen (oder die lokale openapi.yaml verwenden)
2. Den Python-Client-Code generieren
3. Die Verzeichnisstruktur abflachen
4. Überflüssige Konfigurationsdateien bereinigen