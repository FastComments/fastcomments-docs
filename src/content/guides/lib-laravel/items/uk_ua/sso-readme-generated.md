Увімкніть SSO у вашому `.env`:

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

API-ключ потрібен для режиму secure SSO — він використовується для підписування SSO-даних.

### Відображення на основі конфігурації

У файлі `config/fastcomments.php` зіставте поля FastComments з атрибутами вашої моделі User:

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // підтримується точкова нотація
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### Відображення на основі інтерфейсу

Щоб мати більший контроль, реалізуйте інтерфейс `MapsToFastCommentsUser` у вашій моделі User:

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

Коли інтерфейс реалізовано, він має пріоритет над відображенням, заданим у конфігурації.

### SSO у Blade

Коли SSO увімкнено, компонент `<x-fastcomments />` автоматично додає SSO-дані для автентифікованого користувача.