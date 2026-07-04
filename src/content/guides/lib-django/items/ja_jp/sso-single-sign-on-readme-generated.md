Enable SSO and choose a mode in `settings.py`. Secure SSO signs the user
server-side with HMAC‑SHA256 using your API secret and is recommended.

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # あなたのAPIシークレット; Secure SSOに署名します
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # FastComments のフィールドをユーザーモデルにマップします。値は属性
        # 名、ドット区切りパス（"profile.avatar_url"）、callable(user)、または None です。
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool、またはドット区切りパス
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list、またはドット区切りパス
    },
}
```

> **SSO の `id` を慎重に選択してください。** FastComments の `id` はユーザーのコメント履歴の永続的なハンドルです。デフォルトの `USER_MAP` は設定不要の便利さのためにあなたの Django の主キーにマップしますが、連続した整数 PK は列挙可能で後で変更が困難です（ユーザーの `id` を変更すると履歴が新しいアカウントに分割されます）。デモを超えるすべての場合、`id` を事前に選択した安定した不透明な値（UUID または専用の公開 ID）にマップし、決してプライベートデータを格納しないでください。この理由でサンプルアプリはユーザー名ベースの id を使用しています。

SSO は現在のユーザーに対して `{% fastcomments %}`, `{% fastcomments_live_chat %}`,
`{% fastcomments_collab_chat %}`, `{% fastcomments_image_chat %}`, および
`{% fastcomments_user_activity %}` に自動的に注入されます。

Login/logout URLs shown to signed-out visitors default to `reverse("login")` /
`reverse("logout")`; override them with `SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]`.

### カスタムマッピング

`USER_MAP` よりも優先度の高いオプションが2つあります：

- **ユーザーモデルのメソッド**（インターフェースの Python的類似）:

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **グローバルマッパー**、`callable(user) -> dict` へのドット区切りパス:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

優先順位は `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP` です。