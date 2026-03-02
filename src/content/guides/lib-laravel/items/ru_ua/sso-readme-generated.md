Включите SSO в вашем `.env`:

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

API-ключ требуется для secure SSO — он используется для подписи SSO-пейлоада.

### Отображение на основе конфигурации

В `config/fastcomments.php` сопоставьте поля FastComments с атрибутами вашей модели User:

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // поддерживается точечная нотация
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### Отображение через интерфейс

Для большего контроля реализуйте интерфейс `MapsToFastCommentsUser` в вашей модели User:

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

Когда интерфейс реализован, он имеет приоритет над отображением, основанным на конфигурации.

### SSO в Blade

Когда SSO включен, компонент `<x-fastcomments />` автоматически вставляет SSO-данные для аутентифицированного пользователя.