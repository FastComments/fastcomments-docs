Активирайте SSO във вашия `.env`:

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

API ключът е задължителен за защитеното SSO — той се използва за подписване на SSO полезния товар.

### Картографиране чрез конфигурация

В `config/fastcomments.php`, свържете полетата на FastComments с атрибутите на вашия User модел:

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // поддържа се точкова нотация
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### Картографиране чрез интерфейс

За повече контрол, реализирайте интерфейса `MapsToFastCommentsUser` в модела User:

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

Когато интерфейсът е реализиран, той има предимство пред картографирането, базирано на конфигурацията.

### SSO в Blade

Когато SSO е активирано, компонентът `<x-fastcomments />` автоматично вмъква SSO данни за автентикирания потребител.