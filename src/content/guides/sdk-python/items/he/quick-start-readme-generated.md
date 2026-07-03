### Using Authenticated APIs (DefaultApi)

**חשוב:** עליך להגדיר את מפתח ה‑API שלך ב‑Configuration לפני ביצוע בקשות מאומתות. אם לא תעשה זאת, הבקשות יכשלו עם שגיאת 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# יצור והגדרת לקוח ה‑API
config = Configuration()
config.host = "https://fastcomments.com"

# חובה: הגדר את מפתח ה‑API שלך (קבל זאת מלוח הבקרה של FastComments)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# צור את מופע ה‑API עם הלקוח המוגדר
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# עכשיו ניתן לבצע קריאות API מאומתות
try:
    # דוגמה: הוספת משתמש SSO
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # שגיאות נפוצות:
    # - 401: מפתח ה‑API חסר או לא תקין
    # - 400: אימות הבקשה נכשל
```

### Using Public APIs (PublicApi)

Public endpoints don't require authentication:

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

### Using the Moderation Dashboard (ModerationApi)

The `ModerationApi` powers the moderator dashboard. Methods are called on behalf of a moderator by passing an `sso` token:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # ספירת ההערות שממתינות למודעות
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Using SSO (Single Sign-On)

The SDK includes utilities for generating secure SSO tokens:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# יצירת נתוני משתמש (id, email, ו‑username נדרשים)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# חתימה עם סוד ה‑API שלך (HMAC‑SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# יצירת טוקן SSO להעברה לווידג'ט או לקריאת API
sso_token = sso.create_token()

# השתמש בטוקן זה בממשק הקדמי או העבר אותו לקריאות API
print(f"SSO Token: {sso_token}")
```

For simple SSO (less secure, for testing):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### Common Issues

1. **שגיאת 401 "missing-api-key":** ודא שהגדרת `config.api_key = {"api_key": "YOUR_KEY"}` לפני יצירת מופע ה‑DefaultApi.
2. **מחלקת API שגויה:** השתמש ב‑`DefaultApi` לבקשות מאומתות בצד השרת, ב‑`PublicApi` לבקשות ציבוריות/צד הלקוח, וב‑`ModerationApi` לבקשות מדף המודרטור.
3. **שגיאות ייבוא:** ודא שאתה מייבא מהמודול הנכון:
   - לקוח API: `from client import ...`
   - כלי SSO: `from sso import ...`