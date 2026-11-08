`settings.py` में SSO सक्षम करें और एक मोड चुनें। Secure SSO आपके API secret का उपयोग करके HMAC-SHA256 के साथ यूज़र को सर्वर-साइड पर साइन करता है और यही अनुशंसित है।

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # आपका API secret; Secure SSO साइन करता है
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # FastComments फ़ील्ड्स को अपने यूज़र मॉडल से मैप करें। मान एक attribute
        # नाम, एक dotted path ("profile.avatar_url"), एक callable(user), या None हो सकते हैं।
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool, या dotted path
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list, या dotted path
    },
}
```

> **SSO `id` को सोच-समझकर चुनें।** FastComments `id` यूज़र के कमेंट हिस्ट्री का स्थायी हैंडल है। डिफ़ॉल्ट `USER_MAP` इसे zero-config सुविधा के लिए आपकी Django primary key से मैप करता है, लेकिन क्रमागत integer PKs enumerable होते हैं और बाद में बदलना कठिन है (यूज़र का `id` बदलने से उनका हिस्ट्री एक नए अकाउंट में बंट जाता है)। डेमो से आगे किसी भी चीज़ के लिए, `id` को पहले से चुने गए एक स्थिर, अपारदर्शी मान (एक UUID या एक समर्पित public id) से मैप करें, और उसमें कभी निजी डेटा न रखें। उदाहरण ऐप इसी कारण से username-आधारित id का उपयोग करता है।

SSO वर्तमान यूज़र के लिए `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}`, और
`{% fastcomments_user_activity %}` में स्वचालित रूप से इंजेक्ट हो जाता है।

साइन-आउट विज़िटर्स को दिखाए जाने वाले login/logout URLs डिफ़ॉल्ट रूप से `reverse("login")` /
`reverse("logout")` होते हैं; इन्हें `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]` से ओवरराइड करें।

### कस्टम मैपिंग

दो उच्च-प्राथमिकता विकल्प `USER_MAP` को मात देते हैं:

- **आपके यूज़र मॉडल पर एक मेथड** (एक इंटरफ़ेस का Pythonic एनालॉग):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **एक ग्लोबल मैपर**, `callable(user) -> dict` का एक dotted path:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

प्राथमिकता क्रम है `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`।