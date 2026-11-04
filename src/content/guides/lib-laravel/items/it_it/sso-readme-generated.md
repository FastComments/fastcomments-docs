Abilita SSO nel tuo `.env`:

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

La chiave API è necessaria per l'SSO sicuro — viene utilizzata per firmare il payload SSO.

### Mappatura Basata su Config

In `config/fastcomments.php`, mappa i campi di FastComments sugli attributi del tuo modello User:

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // notazione a puntini supportata
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### Mappatura Basata su Interfaccia

Per un maggiore controllo, implementa l'interfaccia `MapsToFastCommentsUser` sul tuo modello User:

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

Quando l'interfaccia è implementata, ha la precedenza sulla mappatura basata su config.

### SSO in Blade

Quando SSO è abilitato, il componente `<x-fastcomments />` inietta automaticamente i dati SSO per l'utente autenticato.