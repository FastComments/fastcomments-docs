Aktivér SSO i din `.env`:

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

API-nøglen er påkrævet for secure SSO — den bruges til at signere SSO-payloaden.

### Konfigurationsbaseret kortlægning

I `config/fastcomments.php`, kortlæg FastComments-felter til dine User-modelattributter:

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // punktnotation understøttet
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### Interface-baseret kortlægning

For mere kontrol, implementer `MapsToFastCommentsUser`-interfacet på din User-model:

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

Når interfacet er implementeret, har det forrang frem for konfigurationsbaseret kortlægning.

### SSO i Blade

Når SSO er aktiveret, injicerer `<x-fastcomments />`-komponenten automatisk SSO-data for den autentificerede bruger.