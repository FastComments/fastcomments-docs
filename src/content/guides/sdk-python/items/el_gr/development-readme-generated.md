### Εκτέλεση δοκιμών

```bash
# Ρύθμιση μεταβλητών περιβάλλοντος
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"

# Εκτέλεση δοκιμών
pytest tests/
```

### Αναδημιουργία του client

Για να αναδημιουργήσετε τον API client από την τελευταία προδιαγραφή OpenAPI:

```bash
./update.sh
```

Αυτό θα:
1. Θα κατεβάσει την πιο πρόσφατη προδιαγραφή OpenAPI από έναν τρέχοντα FastComments server (ή θα χρησιμοποιήσει το τοπικό openapi.yaml)
2. Θα δημιουργήσει τον κώδικα του Python client
3. Θα επιπεδοποιήσει τη δομή των καταλόγων
4. Θα καθαρίσει περιττά αρχεία διαμόρφωσης