---
Włącz SSO w swoim pliku `.env`:

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

Klucz API jest wymagany dla bezpiecznego SSO — służy do podpisywania danych SSO.

### Mapowanie oparte na konfiguracji

W pliku `config/fastcomments.php` zmapuj pola FastComments na atrybuty modelu User:

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // obsługa notacji kropkowej
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### Mapowanie oparte na interfejsie

Dla większej kontroli zaimplementuj interfejs `MapsToFastCommentsUser` w swoim modelu User:

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

Gdy interfejs jest zaimplementowany, ma on pierwszeństwo przed mapowaniem opartym na konfiguracji.

### SSO w Blade

Gdy SSO jest włączone, komponent `<x-fastcomments />` automatycznie wstrzykuje dane SSO dla uwierzytelnionego użytkownika.
---