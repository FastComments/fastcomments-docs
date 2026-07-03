### Χρήση Πιστοποιημένων API (DefaultApi)

**Σημαντικό:** Πρέπει να ορίσετε το κλειδί API σας στη Configuration πριν κάνετε πιστοποιημένα αιτήματα. Αν δεν το κάνετε, τα αιτήματα θα αποτύχουν με σφάλμα 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Δημιουργία και διαμόρφωση του πελάτη API
config = Configuration()
config.host = "https://fastcomments.com"

# ΑΠΑΙΤΟΥΜΕΝΟ: Ορίστε το κλειδί API σας (λάβετε το από τον πίνακα ελέγχου FastComments)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# Δημιουργία της παρουσίασης API με τον διαμορφωμένο πελάτη
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Τώρα μπορείτε να κάνετε πιστοποιημένες κλήσεις API
try:
    # Παράδειγμα: Προσθήκη χρήστη SSO
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Συχνά σφάλματα:
    # - 401: Το κλειδί API λείπει ή δεν είναι έγκυρο
    # - 400: Η έγκριση του αιτήματος απέτυχε
```

### Χρήση Δημόσιων API (PublicApi)

Τα δημόσια σημεία τέλους δεν απαιτούν πιστοποίηση:

```python
from client import ApiClient, Configuration, PublicApi

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public("YOUR_TENANT_ID", "page-url-id")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Χρήση του Πίνακα Διαχείρισης Εποπτείας (ModerationApi)

Το `ModerationApi` τροφοδοτεί τον πίνακα ελεγκτή. Οι μέθοδοι καλούνται εκ μέρους ενός ελεγκτή περνώντας ένα διακριτικό `sso`:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Καταμέτρηση των σχολίων που περιμένουν εποπτεία
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Χρήση SSO (Μονή Σύνδεση)

Το SDK περιλαμβάνει βοηθητικά εργαλεία για τη δημιουργία ασφαλών διακριτικών SSO:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Δημιουργία δεδομένων χρήστη (απαιτούνται id, email και username)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Υπογράψτε το με το μυστικό API σας (HMAC-SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# Δημιουργήστε το διακριτικό SSO για να το περάσετε στο widget ή σε κλήση API
sso_token = sso.create_token()

# Χρησιμοποιήστε αυτό το διακριτικό στο frontend ή περάστε το σε κλήσεις API
print(f"SSO Token: {sso_token}")
```

Για απλό SSO (λιγότερο ασφαλές, για δοκιμές):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Συχνά Προβλήματα

1. **Σφάλμα 401 "missing-api-key"**: Βεβαιωθείτε ότι έχετε ορίσει `config.api_key = {"api_key": "YOUR_KEY"}` πριν δημιουργήσετε την παρουσία DefaultApi.
2. **Λάθος κλάση API**: Χρησιμοποιήστε `DefaultApi` για αιτήματα πιστοποιημένα στο διακομιστή, `PublicApi` για αιτήματα πελάτη/δημόσια, και `ModerationApi` για αιτήματα πίνακα ελεγκτή.
3. **Σφάλματα εισαγωγής**: Βεβαιωθείτε ότι εισάγετε από το σωστό μοντέλο:
   - Πελάτης API: `from client import ...`
   - Βοηθητικά SSO: `from sso import ...`