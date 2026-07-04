| Key | Default | Description |
|---|---|---|
| `TENANT_ID` | `""` | Το tenant ID του FastComments (`demo` για δοκιμές). |
| `API_KEY` | `""` | Το μυστικό API σας. Υπογράφει Secure SSO και πιστοποιεί το `admin()`. |
| `REGION` | `None` | `None` για US, `"eu"` για την περιοχή της ΕΕ. |
| `SSO.ENABLED` | `False` | Ενεργοποιήστε το SSO. |
| `SSO.MODE` | `"secure"` | `"secure"` (HMAC) ή `"simple"` (χωρίς υπογραφή). |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | Εμφανίζεται σε επισκέπτες που έχουν αποσυνδεθεί· προεπιλογή είναι `reverse("login"/"logout")`. |
| `SSO.USER_MAP` | id/email/username | Πεδίο FastComments προς attribute/path/callable του χρήστη. |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` ή διαδρομή με τελείες. |
| `SSO.USER_MAPPER` | `None` | Διαδρομή με τελείες προς `callable(user) -> dict`; υψηλότερη προτεραιότητα. |
| `WIDGET_DEFAULTS` | `{}` | Διαμόρφωση που συγχωνεύεται σε κάθε widget (κλειδιά camelCase). |