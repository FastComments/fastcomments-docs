### Using Authenticated APIs (DefaultApi)

**Σημαντικό:** Πρέπει να ορίσετε το κλειδί API σας στη Διαμόρφωση (Configuration) πριν κάνετε πιστοποιημένα αιτήματα. Αν δεν το κάνετε, τα αιτήματα θα αποτύχουν με σφάλμα 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Δημιουργία και διαμόρφωση του πελάτη API
config = Configuration()
config.host = "https://fastcomments.com/api"

# ΑΠΑΙΤΟΥΜΕΝΟ: Ορίστε το κλειδί API σας (ανακτήστε το από τον πίνακα ελέγχου FastComments)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Δημιουργία της διεπαφής API με τον διαμορφωμένο πελάτη
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
    # - 404: Το κλειδί API λείπει ή είναι άκυρο
    # - 400: Η επικύρωση του αιτήματος απέτυχε
```

### Using Public APIs (PublicApi)

Τα δημόσια σημεία πρόσβασης δεν απαιτούν πιστοποίηση:

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

### Using the Moderation Dashboard (ModerationApi)

Το `ModerationApi` τροφοδοτεί τον πίνακα ελέγχου του συντονιστή. Οι μέθοδοι καλούνται εκ μέρους ενός συντονιστή περνώντας ένα διακριτικό `sso`:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Καταμέτρηση των σχολίων που περιμένουν εποπτεία
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Using SSO (Single Sign-On)

Το SDK περιλαμβάνει εργαλεία για τη δημιουργία ασφαλών διακριτικών SSO:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Δημιουργία δεδομένων χρήστη
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Δημιουργία παραδείγματος SSO με το μυστικό API σας
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Δημιουργία του διακριτικού SSO
sso_token = sso.create_token()

# Χρησιμοποιήστε αυτό το διακριτικό στο frontend σας ή περάστε το σε κλήσεις API
print(f"SSO Token: {sso_token}")
```

For simple SSO (less secure, for testing):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    user_id="user-123",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Common Issues

1. **401 "missing-api-key" error**: Βεβαιωθείτε ότι έχετε ορίσει `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` πριν δημιουργήσετε το στιγμιότυπο DefaultApi.
2. **Wrong API class**: Χρησιμοποιήστε `DefaultApi` για αιτήματα πιστοποιημένα από τον διακομιστή, `PublicApi` για αιτήματα από τον πελάτη/δημόσια, και `ModerationApi` για αιτήματα πίνακα ελέγχου συντονιστή.
3. **Import errors**: Βεβαιωθείτε ότι εισάγετε από το σωστό module:
   - **Πελάτης API**: `from client import ...`
   - **Εργαλεία SSO**: `from sso import ...`