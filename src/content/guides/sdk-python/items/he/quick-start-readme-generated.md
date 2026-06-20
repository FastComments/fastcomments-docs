### שימוש ב‑APIs מאומתים (DefaultApi)

**חשוב:** עליך להגדיר את מפתח ה‑API ב‑Configuration לפני ביצוע קריאות מאומתות. אם לא תעשה זאת, הקריאות ייכשלו עם שגיאת 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# Create and configure the API client
config = Configuration()
config.host = "https://fastcomments.com/api"

# REQUIRED: Set your API key (get this from your FastComments dashboard)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# Create the API instance with the configured client
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# Now you can make authenticated API calls
try:
    # Example: Add an SSO user
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
    # Common errors:
    # - 401: API key is missing or invalid
    # - 400: Request validation failed
```

### שימוש ב‑APIs ציבוריים (PublicApi)

נקודות קצה ציבוריות אינן דורשות אימות:

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

### שימוש בלוח הבקרה של המודרציה (ModerationApi)

The `ModerationApi` powers the moderator dashboard. Methods are called on behalf of a moderator by passing an `sso` token:

```python
from client import ApiClient, Configuration, ModerationApi

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # Count the comments awaiting moderation
    response = moderation_api.get_count(sso="SSO_TOKEN")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### שימוש ב‑SSO (Single Sign-On)

ה‑SDK כולל כלי עזר ליצירת אסימוני SSO מאובטחים:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# Create user data
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# Create SSO instance with your API secret
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# Generate the SSO token
sso_token = sso.create_token()

# Use this token in your frontend or pass to API calls
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

### בעיות נפוצות

1. **401 "missing-api-key" error**: ודא שהגדרת `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` לפני יצירת מופע DefaultApi.
2. **Wrong API class**: השתמש ב‑`DefaultApi` עבור קריאות מאומתות בצד השרת, ב‑`PublicApi` עבור קריאות מצד הלקוח/ציבוריות, וב‑`ModerationApi` עבור קריאות של לוח הבקרה של המודרטור.
3. **Import errors**: ודא שאתה מייבא מהמודול הנכון:
   - לקוח ה‑API: `from client import ...`
   - כלי SSO: `from sso import ...`