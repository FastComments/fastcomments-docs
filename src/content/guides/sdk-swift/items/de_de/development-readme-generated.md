### Client neu generieren

Um den API-Client aus der neuesten OpenAPI-Spezifikation neu zu generieren:

1. Stellen Sie sicher, dass der FastComments-Server lokal unter `http://localhost:3001` läuft
2. Führen Sie das Update-Skript aus:

```bash
./update.sh
```

Dies wird:
- Lädt die neueste OpenAPI-Spezifikation herunter
- Generiert den Swift-Clientcode (mit API-Dokumentation in client/docs)
- Baut das Paket, um zu überprüfen, dass alles funktioniert