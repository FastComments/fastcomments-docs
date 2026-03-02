`.env`でSSOを有効にする:

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

セキュアSSOにはAPIキーが必要です — SSOペイロードの署名に使用されます。

### 設定ベースのマッピング

`config/fastcomments.php`で、FastCommentsのフィールドをUserモデルの属性にマッピングします:

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // dot notation supported
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### インターフェースベースのマッピング

より細かく制御するには、Userモデルに`MapsToFastCommentsUser`インターフェースを実装してください:

```php
use FastComments\Laravel\SSO\Contracts\MapsToFastCommentsUser;

class User extends Authenticatable implements MapsToFastCommentsUser
{
    public function toFastCommentsUserData(): array
    {
        return [
            'id' => (string) $this->id,
            'email' => $this->email,
            'username' => $this->display_name,
            'avatar' => $this->avatar_url,
            'is_admin' => $this->hasRole('admin'),
        ];
    }
}
```

インターフェースを実装すると、設定ベースのマッピングより優先されます。

### BladeでのSSO

SSOが有効な場合、<x-fastcomments />コンポーネントは認証済みユーザーのSSOデータを自動的に注入します。