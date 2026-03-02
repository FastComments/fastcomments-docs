Omogućite SSO u vašem `.env`:

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

API ključ je obavezan za sigurni SSO — koristi se za potpisivanje SSO payload-a.

### Mapiranje zasnovano na konfiguraciji

U `config/fastcomments.php`, mapirajte FastComments polja na atribute vašeg User modela:

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // podržana tačkasta notacija
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### Mapiranje putem interfejsa

Za veću kontrolu, implementirajte interfejs `MapsToFastCommentsUser` na vašem User modelu:

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

Kada je interfejs implementiran, on ima prednost nad mapiranjem zasnovanim na konfiguraciji.

### SSO u Blade

Kada je SSO omogućen, komponenta `<x-fastcomments />` automatski ubacuje SSO podatke za autentifikovanog korisnika.