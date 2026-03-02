---
Activez le SSO dans votre `.env` :

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

La clé API est requise pour le SSO sécurisé — elle est utilisée pour signer la charge utile SSO.

### Mappage basé sur la configuration

Dans `config/fastcomments.php`, mappez les champs FastComments aux attributs de votre modèle User :

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // la notation par points est prise en charge
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### Mappage basé sur l'interface

Pour un contrôle plus fin, implémentez l'interface `MapsToFastCommentsUser` sur votre modèle User :

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

Lorsque l'interface est implémentée, elle prend la priorité sur le mappage basé sur la configuration.

### SSO dans Blade

Lorsque le SSO est activé, le composant `<x-fastcomments />` injecte automatiquement les données SSO pour l'utilisateur authentifié.
---