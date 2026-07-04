Enable SSO ובחרו מודל ב‑`settings.py`. Secure SSO חותמת על המשתמש בצד ה‑שרת עם HMAC‑SHA256 באמצעות סוד ה‑API שלכם ומומלצת.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # סוד ה‑API שלכם; חותמת Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # ממפות שדות FastComments למודל המשתמש שלכם. ערכים יכולים להיות attribute
        # name, נתיב מנוקד ("profile.avatar_url"), callable(user), או None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, או נתיב מנוקד
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, או נתיב מנוקד
    },
}
```

> **בחרו בכוונה את `id` של SSO.** ה‑`id` של FastComments הוא המזהה הקבוע  
> עבור היסטוריית ההערות של משתמש. ה‑`USER_MAP` ברירת המחדל ממפה אותו למפתח הראשי של Django שלכם לנוחות ללא קונפיגורציה, אך מפתחות שלמים רצופים ניתנים לה‑enumeration וקשה לשנותם מאוחר יותר (שינוי `id` של משתמש מפצל את ההיסטוריה שלו לחשבון חדש). עבור כל דבר מעבר לדוגמה, מפתו `id` לערך יציב ולא שקוף שנבחר מראש (UUID או מזהה ציבורי ייעודי), ולעולם אל תכניסו אליו נתונים פרטיים. האפליקציה לדוגמה משתמשת ב‑`id` מבוסס שם משתמש מסיבה זו.

SSO מוזרק אוטומטית ל‑`{% fastcomments %}`, `{% fastcomments_live_chat %}`,  
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}`, ו‑`{% fastcomments_user_activity %}` עבור המשתמש הנוכחי.

כתובות URL של כניסה/יציאה שמוצגות למבקרים שלא מחוברים בברירת מחדל הן `reverse("login")` /  
`reverse("logout")`; ניתן לשנותן באמצעות `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### מיפוי מותאם אישית

שתי אפשרויות בעלות עדיפות גבוהה יותר גוברות על `USER_MAP`:

- **מתודה במודל המשתמש שלכם** (האנלוגיה הפייתונית לממשק):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **ממפה גלובלית**, נתיב מנוקד ל‑`callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

העדיפות היא `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.