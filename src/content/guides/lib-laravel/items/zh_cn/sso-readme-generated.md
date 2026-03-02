---
在你的 `.env` 中启用 SSO：

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

API 密钥是 secure SSO 所必需的 —— 它用于对 SSO 有效负载进行签名。

### 基于配置的映射

在 `config/fastcomments.php` 中，将 FastComments 字段映射到你的 User 模型属性：

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // 支持点表示法
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### 基于接口的映射

若需更精细的控制，在你的 User 模型上实现 `MapsToFastCommentsUser` 接口：

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

当实现该接口时，它会优先于基于配置的映射。

### Blade 中的 SSO

当启用 SSO 时，`<x-fastcomments />` 组件会自动为已认证用户注入 SSO 数据。
---