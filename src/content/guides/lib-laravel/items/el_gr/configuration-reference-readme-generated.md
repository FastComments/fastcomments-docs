---
| Κλειδί | Μεταβλητή περιβάλλοντος | Προεπιλογή | Περιγραφή |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | Το tenant ID του FastComments σας |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | Κλειδί API για κλήσεις στην πλευρά του διακομιστή |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (ΗΠΑ) ή `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | Ενεργοποίηση SSO |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` ή `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | URL σύνδεσης (εάν δεν δοθεί, χρησιμοποιεί τη διαδρομή Laravel) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | URL αποσύνδεσης (εάν δεν δοθεί, χρησιμοποιεί τη διαδρομή Laravel) |
| `widget_defaults` | — | `[]` | Προεπιλεγμένες επιλογές διαμόρφωσης του widget |
---