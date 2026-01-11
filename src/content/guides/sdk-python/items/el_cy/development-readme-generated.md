### Εκτέλεση δοκιμών

```bash
# Ρύθμιση μεταβλητών περιβάλλοντος
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"

# Εκτέλεση δοκιμών
pytest tests/
```

### Επαναδημιουργία του API client

Για να επαναδημιουργήσετε τον API client από την πιο πρόσφατη προδιαγραφή OpenAPI:

```bash
./update.sh
```

Αυτό θα:
1. Κατεβάζει την πιο πρόσφατη προδιαγραφή OpenAPI από έναν ενεργό διακομιστή FastComments (ή χρησιμοποιεί το τοπικό openapi.yaml)
2. Δημιουργεί τον κώδικα του Python client
3. Επιπεδοποιεί τη δομή των καταλόγων
4. Καθαρίζει τα περιττά αρχεία ρυθμίσεων
---