---
Schakel SSO in je `.env`-bestand in:

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

De API-sleutel is vereist voor beveiligde SSO — deze wordt gebruikt om de SSO-payload te ondertekenen.

### Config-gebaseerde mapping

In `config/fastcomments.php` map je FastComments-velden naar de attributen van je User-model:

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // puntnotatie ondersteund
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### Interface-gebaseerde mapping

Voor meer controle implementeer je de `MapsToFastCommentsUser`-interface op je User-model:

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

Wanneer de interface is geïmplementeerd, heeft deze voorrang boven de config-gebaseerde mapping.

### SSO in Blade

Wanneer SSO is ingeschakeld, injecteert de `<x-fastcomments />`-component automatisch SSO-gegevens voor de geauthenticeerde gebruiker.
---