---
SSO in Ihrer `.env` aktivieren:

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

Der API-Schlüssel wird für sicheres SSO benötigt — er wird verwendet, um die SSO-Nutzlast zu signieren.

### Konfigurationsbasierte Zuordnung

In `config/fastcomments.php` ordnen Sie FastComments-Felder den Attributen Ihres User-Modells zu:

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // Punktnotation wird unterstützt
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### Schnittstellenbasierte Zuordnung

Für mehr Kontrolle implementieren Sie das Interface `MapsToFastCommentsUser` in Ihrem User-Modell:

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

Wenn das Interface implementiert ist, hat es Vorrang vor der konfigurationsbasierten Zuordnung.

### SSO in Blade

Wenn SSO aktiviert ist, injiziert die Komponente `<x-fastcomments />` automatisch SSO-Daten für den authentifizierten Benutzer.
---