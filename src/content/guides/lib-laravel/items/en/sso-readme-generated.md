Enable SSO in your `.env`:

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

The API key is required for secure SSO — it is used to sign the SSO payload.

### Config-Based Mapping

In `config/fastcomments.php`, map FastComments fields to your User model attributes:

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // dot notation supported
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### Interface-Based Mapping

For more control, implement the `MapsToFastCommentsUser` interface on your User model:

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

When the interface is implemented, it takes precedence over config-based mapping.

### SSO in Blade

When SSO is enabled, the `<x-fastcomments />` component automatically injects SSO data for the authenticated user.