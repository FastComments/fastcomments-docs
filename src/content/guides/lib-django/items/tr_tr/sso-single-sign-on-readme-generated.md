SSO'yu etkinleştirin ve `settings.py` içinde bir mod seçin. Secure SSO, API gizli anahtarınızı kullanarak HMAC‑SHA256 ile kullanıcıyı sunucu tarafında imzalar ve önerilir.

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

> **SSO `id`'yi kasıtlı olarak seçin.** FastComments `id`si, bir kullanıcının yorum geçmişi için kalıcı bir tanımlayıcıdır. Varsayılan `USER_MAP`, sıfır yapılandırma kolaylığı için bunu Django birincil anahtarınıza eşler, ancak sıralı tamsayı PK'ları sayılabilir ve daha sonra değiştirilmesi zordur (bir kullanıcının `id`si değiştirildiğinde geçmişi yeni bir hesaba bölünür). Bir demo dışındaki her şey için `id`yi önceden seçilmiş, stabil, opak bir değere (örneğin bir UUID veya özel bir genel kimlik) eşleyin ve içine asla özel veri koymayın. Örnek uygulama bu sebeple kullanıcı adı tabanlı bir id kullanır.

SSO, mevcut kullanıcı için `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}` ve
`{% fastcomments_user_activity %}` içine otomatik olarak enjekte edilir.

Oturum açma/kapama URL'leri, oturum açmamış ziyaretçilere varsayılan olarak `reverse("login")` /
`reverse("logout")` olarak gösterilir; bunları `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]` ile geçersiz kılabilirsiniz.

### Özel eşleme

`USER_MAP`'i geçersiz kılan iki daha yüksek öncelikli seçenek vardır:

- **Kullanıcı modelinizde bir yöntem** (bir arayüzün Pythonik benzeri):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **Küresel eşleyici**, `callable(user) -> dict` tipinde bir noktalı yol:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

Öncelik sırası `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP`.