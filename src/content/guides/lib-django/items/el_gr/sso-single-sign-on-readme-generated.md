Ενεργοποιήστε το SSO και επιλέξτε λειτουργία στο `settings.py`. Το Secure SSO υπογράφει τον χρήστη
server-side με HMAC-SHA256 χρησιμοποιώντας το API secret σας και συνιστάται.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # το API secret σας· υπογράφει το Secure SSO
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # Αντιστοιχίστε τα πεδία του FastComments στο μοντέλο χρήστη σας. Οι τιμές μπορεί να είναι
        # όνομα attribute, dotted path ("profile.avatar_url"), callable(user), ή None.
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, ή dotted path
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, ή dotted path
    },
}
```

> **Επιλέξτε το SSO `id` σκόπιμα.** Το `id` του FastComments είναι η μόνιμη
> ταυτότητα για το ιστορικό σχολίων ενός χρήστη. Το προεπιλεγμένο `USER_MAP` το αντιστοιχίζει στο
> Django primary key σας για ευκολία χωρίς ρυθμίσεις, αλλά οι διαδοχικοί ακέραιοι PKs είναι
> ευρετήασιμοι και δύσκολο να αλλάξουν αργότερα (η αλλαγή του `id` ενός χρήστη διαχωρίζει το
> ιστορικό του σε νέο λογαριασμό). Για οτιδήποτε πέρα από ένα demo, αντιστοιχίστε το `id` σε μια σταθερή,
> αδιαφανή τιμή που επιλέγεται εξαρχής (UUID ή ειδικό public id), και μην βάλετε ποτέ
> ιδιωτικά δεδομένα σε αυτό. Η εφαρμογή παράδειγμα χρησιμοποιεί id βασισμένο σε username για αυτόν τον λόγο.

Το SSO εισάγεται αυτόματα στα `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}`, και
`{% fastcomments_user_activity %}` για τον τρέχοντα χρήστη.

Τα URL login/logout που εμφανίζονται σε επισκέπτες χωρίς σύνδεση έχουν προεπιλογή τα `reverse("login")` /
`reverse("logout")`· παρακάμψτε τα με `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### Προσαρμοσμένη αντιστοίχιση

Δύο επιλογές με υψηλότερη προτεραιότητα υπερισχύουν του `USER_MAP`:

- **Μια μέθοδος στο μοντέλο χρήστη σας** (το Pythonic αντίστοιχο μιας διεπαφής):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Ένας καθολικός mapper**, ένα dotted path σε `callable(user) -> dict`:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Η προτεραιότητα είναι `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.