Aktiviere SSO in deiner `.env`:

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

Der API-Schlüssel ist für sicheres SSO erforderlich — er wird verwendet, um die SSO-Payload zu signieren.

### Config-basiertes Mapping

In `config/fastcomments.php` ordnest du die FastComments-Felder den Attributen deines User-Modells zu:

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // Dot-Notation wird unterstützt
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### Interface-basiertes Mapping

Für mehr Kontrolle implementiere das `MapsToFastCommentsUser`-Interface auf deinem User-Modell:

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

Wenn das Interface implementiert ist, hat es Vorrang vor dem config-basierten Mapping.

### SSO in Blade

Wenn SSO aktiviert ist, injiziert die `<x-fastcomments />`-Komponente automatisch die SSO-Daten für den authentifizierten Benutzer.