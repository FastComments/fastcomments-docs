### Χρήση Πιστοποιημένων API (DefaultApi)

**Σημαντικό:** Πρέπει να ορίσετε το API key σας στο Configuration πριν κάνετε πιστοποιημένες κλήσεις. Αν δεν το κάνετε, οι αιτήσεις θα αποτύχουν με σφάλμα 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Δημιουργήστε και διαμορφώστε τον client του API
config = Configuration()
config.host = "https://fastcomments.com/api"

# ΑΠΑΙΤΕΙΤΑΙ: Ορίστε το API key σας (λάβετε το από τον πίνακα ελέγχου του FastComments)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Δημιουργήστε το στιγμιότυπο του API με τον διαμορφωμένο client
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

    response = api.add_sso_user(
        tenant_id="YOUR_TENANT_ID",
        create_apisso_user_data=user_data
    )
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # Συνήθη σφάλματα:
    # - 401: Το API key λείπει ή είναι άκυρο
    # - 400: Η επικύρωση του αιτήματος απέτυχε
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

### Χρήση SSO (Μοναδική Σύνδεση)

Το SDK περιλαμβάνει εργαλεία για τη δημιουργία ασφαλών SSO token:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Δημιουργήστε τα δεδομένα χρήστη
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Δημιουργήστε το στιγμιότυπο SSO με το API secret σας
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Γεννήστε το token SSO
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

### Συνηθισμένα προβλήματα

1. **401 "missing-api-key" error**: Βεβαιωθείτε ότι έχετε ορίσει `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` πριν δημιουργήσετε το στιγμιότυπο DefaultApi.
2. **Wrong API class**: Χρησιμοποιήστε `DefaultApi` για αιτήσεις που γίνονται από την πλευρά του διακομιστή με αυθεντικοποίηση, και `PublicApi` για αιτήσεις από την πλευρά του πελάτη/δημόσιες αιτήσεις.
3. **Import errors**: Βεβαιωθείτε ότι εισάγετε από το σωστό module:
   - API client: `from client import ...`
   - Βοηθήματα SSO: `from sso import ...`