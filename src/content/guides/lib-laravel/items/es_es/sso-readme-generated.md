---
Habilita SSO en tu `.env`:

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

La clave de API es necesaria para SSO seguro — se utiliza para firmar la carga útil del SSO.

### Mapeo basado en configuración

En `config/fastcomments.php`, asigna los campos de FastComments a los atributos de tu modelo User:

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // notación de puntos soportada
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### Mapeo basado en interfaz

Para tener más control, implementa la interfaz `MapsToFastCommentsUser` en tu modelo User:

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

Cuando se implementa la interfaz, esta tiene prioridad sobre el mapeo basado en configuración.

### SSO en Blade

Cuando SSO está habilitado, el `<x-fastcomments />` component inyecta automáticamente los datos de SSO para el usuario autenticado.
---