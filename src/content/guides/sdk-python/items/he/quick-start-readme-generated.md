### שימוש ב‑APIs מאומתים (DefaultApi)

**חשוב:** עליך להגדיר את מפתח ה‑API שלך ב‑Configuration לפני ביצוע בקשות מאומתות. אם לא תעשה זאת, הבקשות יכשלו עם שגיאת 401.

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# יצירת והגדרת לקוח ה‑API
config = Configuration()
config.host = "https://fastcomments.com"

# נדרש: הגדר את מפתח ה‑API שלך (קבל זאת מלוח הבקרה של FastComments)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# יצירת מופע ה‑API עם הלקוח המוגדר
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# כעת ניתן לבצע קריאות API מאומתות
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
    # - 401: API key is missing or invalid
    # - 400: Request validation failed
```

### שימוש ב‑APIs ציבוריים (PublicApi)

קצות ציבוריים אינם דורשים אימות:

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

### שימוש בלוח מודרציה (ModerationApi)

`ModerationApi` מניע את לוח הבקרה של המודרטור. שיטות נקראות בשם מודרטור על‑ידי העברת אסימון `sso`:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # ספירת ההערות הממתינות למודרציה
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### שימוש ב‑SSO (כניסה חד‑פעמית)

ה‑SDK כולל כלי עזר ליצירת אסימוני SSO מאובטחים:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# יצירת נתוני משתמש (id, email, ו‑username נדרשים)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# חתום עליו עם סוד ה‑API שלך (HMAC‑SHA256)
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# יצירת אסימון SSO להעברה ל‑widget או לקריאת API
sso_token = sso.create_token()

# השתמש באסימון זה בממשק הקדמי שלך או העבר אותו לקריאות API
print(f"SSO Token: {sso_token}")
```

ל‑SSO פשוט (פחות מאובטח, לצרכי בדיקה):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### מנויים חיים (PubSub)

המודול `pubsub` מאפשר לך להירשם לאירועי תגובות בזמן אמת (תגובות חדשות, הצבעות, עריכות, התראות וכו') דרך WebSocket, המשקף את `LiveEventSubscriber` של FastComments Java SDK. הוא דורש את התוספת `pubsub`, שמוסיפה לקוח WebSocket על גבי לקוח ה‑API שנוצר:

```bash
pip install "fastcomments[pubsub] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0"
```

```python
from pubsub import LiveEventSubscriber

subscriber = LiveEventSubscriber()

def handle_live_event(event):
    print(f"Live event: {event.type}")
    if event.comment:
        print(f"  comment: {event.comment.comment}")

result = subscriber.subscribe_to_changes(
    tenant_id_ws="YOUR_TENANT_ID",
    url_id="page-url-id",
    url_id_ws="page-url-id",
    user_id_ws="a-unique-presence-id",  # לדוגמה, UUID עבור סשן זה
    handle_live_event=handle_live_event,
    on_connection_status_change=lambda connected, last_event_time: print(
        f"connected={connected}"
    ),
    region=None,  # הגדר ל-"eu" עבור אזור האיחוד האירופי
)

# ...מאוחר יותר, כאשר אינך רוצה עוד עדכונים:
result.close()
```

המנוי מריץ את החיבור על גבי תהליך רקע (daemon), מתחבר מחדש באופן שקוף עם jitter, ומושך כל אירוע שנפספס בזמן שהחיבור נותק מנק מנקודת הקצה של יומן האירועים בעת החיבור מחדש. העבר קריאת קולבק אופציונלית `can_see_comments` (`List[str] -> Dict[str, str]`), המחזירה את המזהים שהמשתמש אינו יכול לראות, כדי לסנן אירועים עבור תגובות שהמשתמש אינו מורשה לצפות בהן. הגדר `disable_live_commenting=True` כדי להפוך את `subscribe_to_changes` לפעולה ריקה שמחזירה `None`.

### בעיות נפוצות

1. **שגיאת 401 "missing-api-key"**: ודא שהגדרת `config.api_key = {"api_key": "YOUR_KEY"}` לפני יצירת מופע DefaultApi.
2. **מחלקת API שגויה**: השתמש ב‑`DefaultApi` לבקשות מאומתות בצד השרת, ב‑`PublicApi` לבקשות צד‑לקוח/ציבוריות, וב‑`ModerationApi` לבקשות ללוח המודרטור.
3. **שגיאות ייבוא**: ודא שאתה מייבא מהמודול הנכון:
   - לקוח API: `from client import ...`
   - כלי SSO: `from sso import ...`
   - מנויים חיים: `from pubsub import ...` (דורש את התוספת `pubsub`)

---