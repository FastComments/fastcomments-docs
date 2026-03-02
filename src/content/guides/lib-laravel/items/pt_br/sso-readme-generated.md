Habilite SSO no seu `.env`:

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

A chave de API é necessária para SSO seguro — ela é usada para assinar o payload de SSO.

### Mapeamento baseado em configuração

Em `config/fastcomments.php`, mapeie os campos do FastComments para os atributos do seu modelo User:

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // notação de ponto suportada
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### Mapeamento baseado em Interface

Para mais controle, implemente a interface `MapsToFastCommentsUser` no seu modelo User:

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

Quando a interface for implementada, ela tem precedência sobre o mapeamento baseado em configuração.

### SSO no Blade

Quando o SSO está habilitado, o `<x-fastcomments />` component automaticamente injeta os dados de SSO para o usuário autenticado.