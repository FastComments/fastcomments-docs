`settings.py` でSSOを有効にし、モードを選択します。Secure SSOはAPIシークレットを使用してHMAC-SHA256でサーバー側からユーザーに署名する方式で、推奨されています。

```python
FASTCOMMENTS = {
    "TENANT_ID": os.environ["FASTCOMMENTS_TENANT_ID"],
    "API_KEY": os.environ["FASTCOMMENTS_API_KEY"],   # APIシークレット。Secure SSOの署名に使用
    "SSO": {
        "ENABLED": True,
        "MODE": "secure",                            # "secure" | "simple"
        # FastCommentsのフィールドをユーザーモデルにマッピングします。値には属性名、
        # ドット区切りのパス("profile.avatar_url")、callable(user)、またはNoneを指定できます。
        "USER_MAP": {
            "id": "id",
            "email": "email",
            "username": "username",
            "avatar": None,
            "display_name": None,
            "website_url": None,
        },
        "IS_ADMIN": lambda user: user.is_staff,      # callable(user) -> bool、またはドット区切りのパス
        "IS_MODERATOR": None,
        "GROUP_IDS": None,                           # callable(user) -> list、またはドット区切りのパス
    },
}
```

> **SSOの `id` は意図的に選択してください。** FastCommentsの `id` は、ユーザーのコメント履歴を示す永続的な識別子です。デフォルトの `USER_MAP` では設定不要の手軽さのためにDjangoのプライマリキーにマッピングされますが、連番の整数PKは推測可能であり、後から変更するのは困難です(ユーザーの `id` を変更すると、履歴が新しいアカウントに分割されてしまいます)。デモ以外の用途では、`id` を最初から選択した安定した不透明な値(UUIDや専用の公開id)にマッピングし、プライベートなデータを含めないでください。サンプルアプリがユーザー名ベースのidを使用しているのはこのためです。

SSOは、現在のユーザーに対して `{% fastcomments %}`、`{% fastcomments_live_chat %}`、`{% fastcomments_collab_chat %}`、`{% fastcomments_image_chat %}`、`{% fastcomments_user_activity %}` に自動的に注入されます。

ログアウト状態の訪問者に表示されるログイン/ログアウトURLは、デフォルトで `reverse("login")` / `reverse("logout")` になります。`SSO["LOGIN_URL"]` / `SSO["LOGOUT_URL"]` で上書きできます。

### カスタムマッピング

`USER_MAP` よりも優先される2つの上位オプションがあります:

- **ユーザーモデル上のメソッド**(インターフェースのPython的な相当物):

  ```python
  class User(AbstractUser):
      def to_fastcomments_user_data(self):
          return {"id": self.pk, "email": self.email, "username": self.get_username()}
  ```

- **グローバルマッパー**。`callable(user) -> dict` へのドット区切りのパス:

  ```python
  FASTCOMMENTS = {"SSO": {"USER_MAPPER": "myapp.sso.map_user"}}
  ```

優先順位は `USER_MAPPER` > `to_fastcomments_user_data()` > `USER_MAP` です。