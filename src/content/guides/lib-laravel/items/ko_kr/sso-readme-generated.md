---
당신의 `.env`에서 SSO를 활성화하세요:

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

보안 SSO에는 API 키가 필요합니다 — SSO 페이로드를 서명하는 데 사용됩니다.

### 구성 기반 매핑

`config/fastcomments.php`에서 FastComments 필드를 User 모델 속성에 매핑하세요:

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // 도트 표기법 지원
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### 인터페이스 기반 매핑

더 세밀한 제어를 위해 User 모델에 `MapsToFastCommentsUser` 인터페이스를 구현하세요:

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

인터페이스가 구현되어 있으면 구성 기반 매핑보다 우선합니다.

### Blade에서의 SSO

SSO가 활성화되면 `<x-fastcomments />` 컴포넌트가 인증된 사용자에 대한 SSO 데이터를 자동으로 주입합니다.
---