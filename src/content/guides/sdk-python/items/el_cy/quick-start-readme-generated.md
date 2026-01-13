### Χρήση Αυθεντικοποιημένων API (DefaultApi)

**Σημαντικό:** Πρέπει να ορίσετε το API key σας στο Configuration πριν κάνετε αιτήματα που απαιτούν αυθεντικοποίηση. Αν δεν το κάνετε, τα αιτήματα θα αποτύχουν με σφάλμα 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Δημιουργία και διαμόρφωση του API client
config = Configuration()
config.host = "https://fastcomments.com/api"

# ΑΠΑΡΑΙΤΗΤΟ: Ορίστε το API key σας (πάρτε το από τον πίνακα ελέγχου του FastComments)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Δημιουργία της παρουσίας API με τον διαμορφωμένο client
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Τώρα μπορείτε να κάνετε αιτήματα API με αυθεντικοποίηση
try:
    # Παράδειγμα: Προσθήκη χρήστη SSO
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user(
        tenant_id="YOUR_TENANT_ID",
        create_apisso_user_data=user_data
    )
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Συνήθη σφάλματα:
    # - 401: Λείπει ή είναι άκυρο το API key
    # - 400: Αποτυχία επικύρωσης αιτήματος
```

### Χρήση Δημόσιων API (PublicApi)

Τα δημόσια endpoints δεν απαιτούν αυθεντικοποίηση:

```python
from client import ApiClient, Configuration, PublicApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public(
        tenant_id="YOUR_TENANT_ID",
        url_id="page-url-id"
    )
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Χρήση SSO (Single Sign-On)

Το SDK περιλαμβάνει βοηθητικά εργαλεία για τη δημιουργία ασφαλών SSO tokens:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Δημιουργία δεδομένων χρήστη
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Δημιουργία στιγμιότυπου SSO με το API secret σας
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Δημιουργία του SSO token
sso_token = sso.create_token()

# Χρησιμοποιήστε αυτό το token στο frontend σας ή περάστε το σε κλήσεις API
print(f"SSO Token: {sso_token}")
```

Για απλό SSO (λιγότερο ασφαλές, για δοκιμές):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Συνηθισμένα Προβλήματα

1. **401 "missing-api-key" σφάλμα**: Βεβαιωθείτε ότι έχετε ορίσει `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` πριν δημιουργήσετε το παράδειγμα DefaultApi.
2. **Λάθος κλάση API**: Χρησιμοποιήστε `DefaultApi` για αιτήματα με αυθεντικοποίηση από τον διακομιστή (server-side), `PublicApi` για αιτήματα από τον πελάτη/δημόσια (client-side/public).
3. **Σφάλματα εισαγωγής**: Βεβαιωθείτε ότι κάνετε import από το σωστό module:
   - Πελάτης API: `from client import ...`
   - Εργαλεία SSO: `from sso import ...`