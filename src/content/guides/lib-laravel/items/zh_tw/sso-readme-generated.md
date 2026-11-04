在你的 `.env` 中啟用 SSO:

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

安全 SSO 需要 API 金鑰——它用於簽署 SSO 負載。

### 基於設定的映射

在 `config/fastcomments.php` 中,將 FastComments 欄位映射到你的 User 模型屬性:

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // 支援點號表示法
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### 基於介面的映射

若需要更多控制權,請在你的 User 模型上實作 `MapsToFastCommentsUser` 介面:

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

當實作了該介面時,它將優先於基於設定的映射。

### 在 Blade 中使用 SSO

當啟用 SSO 時,`<x-fastcomments />` 元件會自動為已驗證的使用者注入 SSO 資料。