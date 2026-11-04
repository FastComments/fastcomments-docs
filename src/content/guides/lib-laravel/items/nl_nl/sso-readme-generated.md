Schakel SSO in via je `.env`:

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

De API-sleutel is vereist voor secure SSO — deze wordt gebruikt om de SSO-payload te ondertekenen.

### Configuratie-gebaseerde mapping

In `config/fastcomments.php` koppel je FastComments-velden aan attributen van je User-model:

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // dot-notatie wordt ondersteund
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### Interface-gebaseerde mapping

Voor meer controle implementeer je de `MapsToFastCommentsUser` interface op je User-model:

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

Wanneer de interface is geïmplementeerd, heeft deze voorrang op configuratie-gebaseerde mapping.

### SSO in Blade

Wanneer SSO is ingeschakeld, injecteert de `<x-fastcomments />` component automatisch SSO-data voor de geauthenticeerde gebruiker.