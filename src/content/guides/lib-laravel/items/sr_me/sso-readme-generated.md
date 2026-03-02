Омогућите SSO у вашем `.env`:

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

API кључ је обавезан за сигуран SSO — он се користи за потписивање SSO података.

### Мапирање на основу конфигурације

У `config/fastcomments.php`, мапирајте FastComments поља на атрибуте вашег User модела:

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // нотација са тачкама је подржана
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### Мапирање преко интерфејса

За већу контролу, имплементирајте интерфејс `MapsToFastCommentsUser` на вашем User моделу:

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

Када је интерфејс имплементиран, он има предност над мапирањем заснованим на конфигурацији.

### SSO у Blade

Када је SSO омогућен, компонента `<x-fastcomments />` аутоматски убацује SSO податке за аутентификованог корисника.