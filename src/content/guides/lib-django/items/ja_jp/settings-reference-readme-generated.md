| キー | デフォルト | 説明 |
|---|---|---|
| `TENANT_ID` | `""` | FastComments のテナント ID (`demo` はテスト用)。 |
| `API_KEY` | `""` | API シークレット。Secure SSO に署名し、`admin()` を認証します。 |
| `REGION` | `None` | `None` は米国、`"eu"` は EU リージョンです。 |
| `SSO.ENABLED` | `False` | SSO をオンにします。 |
| `SSO.MODE` | `"secure"` | `"secure"` (HMAC) または `"simple"` (無署名)。 |
| `SSO.LOGIN_URL` / `SSO.LOGOUT_URL` | `None` | サインアウトした訪問者に表示されます。デフォルトは `reverse("login"/"logout")` です。 |
| `SSO.USER_MAP` | id/email/username | FastComments のフィールドをユーザー属性/パス/呼び出し可能オブジェクトにマッピングします。 |
| `SSO.IS_ADMIN` / `IS_MODERATOR` / `GROUP_IDS` | `None` | `callable(user)` またはドット区切りパス。 |
| `SSO.USER_MAPPER` | `None` | `callable(user) -> dict` へのドット区切りパス。最も高い優先度です。 |
| `WIDGET_DEFAULTS` | `{}` | すべてのウィジェットにマージされる設定（camelCase キー）。 |