Enable SSO and choose a mode in `settings.py`. Secure SSO signs the user
server-side with HMAC‑SHA256 using your API secret and is recommended.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # your API secret; signs Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Map FastComments fields to your user model. Values may be an attribute
        # name, a dotted path ("profile.avatar_url"), a callable(user), or None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, or dotted path
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, or dotted path
    },
}
```

> **Επιλέξτε το SSO `id` σκόπιμα.** Το `id` του FastComments είναι ο μόνιμος
> αναγνωριστής για το ιστορικό σχολίων ενός χρήστη. Το προεπιλεγμένο `USER_MAP`
> το αντιστοιχίζει στο πρωτεύον κλειδί του Django για ευκολία χωρίς ρύθμιση,
> αλλά τα διαδοχικά ακέραια PKs είναι αριθμήσιμα και δύσκολα να αλλάξουν αργότερα
> (η αλλαγή του `id` ενός χρήστη χωρίζει το ιστορικό του σε νέο λογαριασμό). Για
> οτιδήποτε πέρα από μια επίδειξη, αντιστοιχίστε το `id` σε μια σταθερή,
> αδιαφανή τιμή που επιλέγεται εκ των προτέρων (ένα UUID ή ένα αφιερωμένο
> δημόσιο id) και μην τοποθετείτε ποτέ ιδιωτικά δεδομένα σε αυτό. Η παράδειγμα
> εφαρμογή χρησιμοποιεί ένα `id` βασισμένο στο όνομα χρήστη για αυτόν τον λόγο.

SSO is injected automatically into `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}`, and
`{% fastcomments_user_activity %}` for the current user.

Login/logout URLs shown to signed-out visitors default to `reverse("login")` / `reverse("logout")`; override them with `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Custom mapping

Two higher-precedence options beat `USER_MAP`:

- **Μια μέθοδος στο μοντέλο χρήστη σας** (the Pythonic analog of an interface):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Ένας παγκόσμιος mapper**, a dotted path to `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Precedence is `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.