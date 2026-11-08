| कुंजी | डिफ़ॉल्ट | विवरण |
|---|---|---|
| `TENANT_ID` | `""` | आपकी FastComments टेनेंट आईडी (परीक्षण के लिए `demo`)। |
| `API_KEY` | `""` | आपका API सीक्रेट। Secure SSO पर हस्ताक्षर करता है और `admin()` को प्रमाणित करता है। |
| `REGION` | `None` | US के लिए `None`, EU क्षेत्र के लिए `"eu"`। |
| `SSO.ENABLED` | `False` | SSO चालू करें। |
| `SSO.MODE` | `"secure"` | `"secure"` (HMAC) या `"simple"` (बिना हस्ताक्षर)। |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | साइन-आउट किए गए विज़िटर को दिखाया जाता है; डिफ़ॉल्ट रूप से `reverse("login"/"logout")`। |
| `SSO.USER_MAP` | id/email/username | FastComments फ़ील्ड से यूज़र एट्रिब्यूट/पाथ/कॉलेबल की मैपिंग। |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` या डॉटेड पाथ। |
| `SSO.USER_MAPPER` | `None` | `callable(user) -> dict` का डॉटेड पाथ; सर्वोच्च प्राथमिकता। |
| `WIDGET_DEFAULTS` | `{}` | हर विजेट में मर्ज किया जाने वाला कॉन्फ़िग (camelCase कुंजियाँ)। |