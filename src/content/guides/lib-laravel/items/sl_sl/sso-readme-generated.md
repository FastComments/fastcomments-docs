Omogočite SSO v svoji `.env`:

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

API ključ je potreben za varni SSO — uporablja se za podpisovanje SSO podatkov.

### Preslikava na podlagi konfiguracije

V `config/fastcomments.php` preslikajte FastComments polja na atribute modela User:

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // podprta točkovna notacija
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### Preslikava prek vmesnika

Za več nadzora implementirajte vmesnik `MapsToFastCommentsUser` na svojem modelu User:

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

Ko je vmesnik implementiran, ima prednost pred preslikavo na podlagi konfiguracije.

### SSO v Blade

Ko je SSO omogočen, komponenta `<x-fastcomments />` samodejno vstavi SSO podatke za overjenega uporabnika.