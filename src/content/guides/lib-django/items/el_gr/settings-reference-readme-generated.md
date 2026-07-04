| Key | Default | Description |
|---|---|---|
| `TENANT_ID` | `""` | Το tenant ID του FastComments (`demo` για δοκιμή). |
| `API_KEY` | `""` | Το μυστικό API σας. Υπογράφει το Secure SSO και πιστοποιεί το `admin()`. |
| `REGION` | `None` | `None` για τις ΗΠΑ, `"eu"` για την περιοχή της ΕΕ. |
| `SSO.ENABLED` | `False` | Ενεργοποιήστε το SSO. |
| `SSO.MODE` | `"secure"` | `"secure"` (HMAC) ή `"simple"` (μη υπογεγραμμένο). |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | Εμφανίζεται στους επισκέπτες που έχουν αποσυνδεθεί· προεπιλογή είναι `reverse("login"/"logout")`. |
| `SSO.USER_MAP` | id/email/username | Πεδίο FastComments προς χαρακτηριστικό/διαδρομή/κλήση χρήστη. |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` ή διαδρομή με τελείες. |
| `SSO.USER_MAPPER` | `None` | Διαδρομή με τελείες προς `callable(user) -> dict`; υψιότερη προτεραιότητα. |
| `WIDGET_DEFAULTS` | `{}` | Διαμόρφωση ενσωματωμένη σε κάθε widget (κλειδιά camelCase). |