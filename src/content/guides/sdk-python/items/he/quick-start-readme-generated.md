### Using Authenticated APIs (DefaultApi)

**חשוב:** עליך לקבוע את מפתח ה-API שלך בתצורה לפני ביצוע בקשות מאומתות. אם לא תעשה זאת, הבקשות יכשלו עם שגיאת 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# צור והגדר את לקוח ה-API
config = Configuration()
config.host = "https://fastcomments.com/api"

# נחוץ: הגדר את מפתח ה-API שלך (קבל זאת מלוח הבקרה של FastComments)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# צור את מופע ה-API עם הלקוח המוגדר
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# עכשיו אתה יכול לבצע קריאות API מאומתות
try:
    # לדוגמה: הוסף משתמש SSO
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # שגאים נפוצים:
    # - 401: מפתח ה-API חסר או לא תקף
    # - 400: אימות הבקשה נכשל
```

### Using Public APIs (PublicApi)

Public endpoints don't require authentication:

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

`ModerationApi` powers the moderator dashboard. Methods are called on behalf of a moderator by passing an `sso` token:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com/api"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # ציין את מספר ההערות במחכה למודרציה
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### Using SSO (Single Sign-On)

The SDK includes utilities for generating secure SSO tokens:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# צור נתוני משתמש
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# צור מופע SSO עם סוד ה-API שלך
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# צור את אסימון ה-SSO
sso_token = sso.create_token()

# השתמש באסימון זה בצד הקליינט או העבר לקריאות ה-API
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

1. **401 "missing-api-key" error**: ודא שהגדרת `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` לפני יצירת מופע DefaultApi.
2. **Wrong API class**: השתמש ב-`DefaultApi` לבקשות מאומתות בצד השרת, `PublicApi` לבקשות ציבוריות/בצד הלקוח, ו-`ModerationApi` לבקשות ללוח המודרציה.
3. **Import errors**: ודא שאתה מייבא מהמודול הנכון:
   - לקוח API: `from client import ...`
   - כלים ל-SSO: `from sso import ...`