### Χρήση Πιστοποιημένων API (DefaultApi)

**Σημαντικό:** Πρέπει να ορίσετε το κλειδί API σας στη Configuration πριν κάνετε πιστοποιημένα αιτήματα. Εάν δεν το κάνετε, τα αιτήματα θα αποτύχουν με σφάλμα 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Δημιουργία και διαμόρφωση του πελάτη API
config = Configuration()
config.host = "https://fastcomments.com/api"

# ΑΠΑΙΤΕΙΤΑΙ: Ορίστε το κλειδί API σας (λάβετε το από τον πίνακα ελέγχου FastComments)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Δημιουργία του στιγμιότυπου API με τον διαμορφωμένο πελάτη
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Τώρα μπορείτε να κάνετε πιστοποιημένα κλήσεις API
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
    # Συνηθισμένα σφάλματα:
    # - 401: Το κλειδί API λείπει ή δεν είναι έγκυρο
    # - 400: Η επικύρωση του αιτήματος απέτυχε
```

### Χρήση Δημοσίων API (PublicApi)

Τα δημόσια σημεία άκρης δεν απαιτούν πιστοποίηση:

```python
from client import ApiClient, Configuration, PublicApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public("YOUR_TENANT_ID", "page-url-id")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Χρήση Πίνακα Εποπτείας (ModerationApi)

Το `ModerationApi` τροφοδοτεί τον πίνακα ελέγχου των συντονιστών. Οι μέθοδοι καλούνται εκ μέρους ενός συντονιστή περνώντας ένα διακριτικό `sso`:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Καταμέτρηση των σχολίων που αναμένουν εποπτεία
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Χρήση SSO (Single Sign-On)

Το SDK περιλαμβάνει βοηθητικές λειτουργίες για τη δημιουργία ασφαλών διακριτικών SSO:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Δημιουργία δεδομένων χρήστη
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Δημιουργία στιγμιότυπου SSO με το μυστικό API σας
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Δημιουργία του διακριτικού SSO
sso_token = sso.create_token()

# Χρήση αυτού του διακριτικού στο frontend σας ή πέρασμά του σε κλήσεις API
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

### Κοινά Προβλήματα

1. **Σφάλμα 401 "missing-api-key"**: Βεβαιωθείτε ότι έχετε ορίσει `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` πριν δημιουργήσετε το στιγμιότυπο DefaultApi.
2. **Λανθασμένη κλάση API**: Χρησιμοποιήστε `DefaultApi` για αιτήματα εξυπηρέτησης με πιστοποίηση, `PublicApi` για αιτήματα από πελάτη/δημόσια, και `ModerationApi` για αιτήματα πίνακα ελέγχου συντονιστών.
3. **Σφάλματα εισαγωγής**: Βεβαιωθείτε ότι κάνετε import από το σωστό module:
   - Πελάτης API: `from client import ...`
   - Βοηθητικά SSO: `from sso import ...`