### שימוש ב-APIs מאומתים (DefaultApi)

**חשוב:** עליך להגדיר את מפתח ה-API ב-Configuration לפני ביצוע בקשות מאומתות. אם לא תעשה זאת, הבקשות ייכשלו עם שגיאת 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# צור והגדר את לקוח ה-API
config = Configuration()
config.host = "https://fastcomments.com/api"

# נדרש: הגדר את מפתח ה-API שלך (קבל אותו מלוח הבקרה של FastComments)
config.api_key = {"ApiKeyAuth": "YOUR_API_KEY_HERE"}

# צור את מופע ה-API עם הלקוח המוגדר
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# עכשיו אתה יכול לבצע קריאות API מאומתות
try:
    # דוגמה: הוסף משתמש SSO
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
    # שגיאות נפוצות:
    # - 401: מפתח ה-API חסר או שגוי
    # - 400: אימות הבקשה נכשל
```

### שימוש ב-APIs ציבוריים (PublicApi)

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

### שימוש ב-SSO (כניסה יחידה)

ערכת הכלים (SDK) כוללת כלי עזר ליצירת טוקני SSO מאובטחים:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# צור נתוני משתמש
user_data = SecureSSOUserData(
    user_id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# צור מופע SSO עם הסוד של ה-API שלך
sso = FastCommentsSSO.new_secure(
    api_secret="YOUR_API_SECRET",
    user_data=user_data
)

# הפק את טוקן ה-SSO
sso_token = sso.create_token()

# השתמש בטוקן זה בצד הלקוח שלך או העבר אותו לקריאות API
print(f"SSO Token: {sso_token}")
```

עבור SSO פשוט (פחות מאובטח, למטרות בדיקה):

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

1. **401 "missing-api-key" error**: וודא שהגדרת `config.api_key = {"ApiKeyAuth": "YOUR_KEY"}` לפני יצירת מופע DefaultApi.
2. **Wrong API class**: השתמש ב-`DefaultApi` עבור בקשות מאומתות בצד השרת, וב-`PublicApi` עבור בקשות בצד הלקוח/ציבוריות.
3. **Import errors**: וודא שאתה מייבא מהממודול הנכון:
   - לקוח ה-API: `from client import ...`
   - כלי SSO: `from sso import ...`