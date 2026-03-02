---
Abilita SSO nel tuo `.env`:

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

La chiave API è richiesta per l'SSO sicuro — viene usata per firmare il payload SSO.

### Mappatura basata sulla configurazione

In `config/fastcomments.php`, mappa i campi di FastComments agli attributi del tuo modello User:

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // notazione a punti supportata
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### Mappatura basata sull'interfaccia

Per un maggiore controllo, implementa l'interfaccia `MapsToFastCommentsUser` nel tuo modello User:

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

Quando l'interfaccia è implementata, ha la precedenza sulla mappatura basata sulla configurazione.

### SSO in Blade

Quando l'SSO è abilitato, il componente `<x-fastcomments />` inserisce automaticamente i dati SSO per l'utente autenticato.
---