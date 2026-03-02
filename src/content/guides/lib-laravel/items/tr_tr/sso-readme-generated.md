SSO'yu `.env` dosyanızda etkinleştirin:

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

Güvenli SSO için API anahtarı gereklidir — SSO yükünü imzalamak için kullanılır.

### Yapılandırma Tabanlı Eşleme

`config/fastcomments.php` içinde, FastComments alanlarını User modelinizin özelliklerine eşleyin:

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // nokta gösterimi desteklenir
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### Arayüz Tabanlı Eşleme

Daha fazla kontrol için, User modelinizde `MapsToFastCommentsUser` arayüzünü uygulayın:

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

Arayüz uygulandığında, yapılandırma tabanlı eşleme üzerinde öncelik kazanır.

### Blade'de SSO

SSO etkinleştirildiğinde, `<x-fastcomments />` bileşeni kimlik doğrulmuş kullanıcı için SSO verilerini otomatik olarak ekler.