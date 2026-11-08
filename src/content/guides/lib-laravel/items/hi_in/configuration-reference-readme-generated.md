| कुंजी | Env वेरिएबल | डिफ़ॉल्ट | विवरण |
|-----|-------------|---------|-------------|
| `tenant_id` | `FASTCOMMENTS_TENANT_ID` | `''` | आपकी FastComments टेनेंट ID |
| `api_key` | `FASTCOMMENTS_API_KEY` | `''` | सर्वर-साइड कॉल के लिए API कुंजी |
| `region` | `FASTCOMMENTS_REGION` | `null` | `null` (US) या `'eu'` |
| `sso.enabled` | `FASTCOMMENTS_SSO_ENABLED` | `false` | SSO सक्षम करें |
| `sso.mode` | `FASTCOMMENTS_SSO_MODE` | `'secure'` | `'secure'` या `'simple'` |
| `sso.login_url` | `FASTCOMMENTS_SSO_LOGIN_URL` | `null` | लॉगिन URL (Laravel रूट पर वापस जाता है) |
| `sso.logout_url` | `FASTCOMMENTS_SSO_LOGOUT_URL` | `null` | लॉगआउट URL (Laravel रूट पर वापस जाता है) |
| `widget_defaults` | — | `[]` | डिफ़ॉल्ट विजेट कॉन्फ़िग विकल्प |