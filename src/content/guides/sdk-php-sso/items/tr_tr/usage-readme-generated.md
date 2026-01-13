### Basit SSO

Basit SSO kullanımı basittir, ancak Güvenli SSO'ya kıyasla daha az güvenlik sağlar:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SimpleSSOUserData;

// Kullanıcı verilerini oluştur
$userData = SimpleSSOUserData::create(
    'username123',
    'user@example.com',
    'https://example.com/avatar.png'
);

// URL tabanlı giriş/çıkış ile
$sso = FastCommentsSSO::createWithUrls(
    null,
    $userData,
    'https://example.com/login',
    'https://example.com/logout'
);

// Veya geri çağırmalar ile
$sso = FastCommentsSSO::createWithCallbacks(
    $userData,
    function($url) { /* Giriş işlemini ele al */ },
    function($url) { /* Çıkış işlemini ele al */ }
);

// FastComments'a iletmek için token'ı al
$token = $sso->prepareToSend();
```

### Güvenli SSO

Güvenli SSO, HMAC doğrulaması ile artırılmış güvenlik sağlar:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SecureSSOUserData;

// Kullanıcı verilerini oluştur
$userData = SecureSSOUserData::create(
    'user-123',
    'user@example.com',
    'username123',
    'https://example.com/avatar.png'
);

// Gerekirse isteğe bağlı veriler ekleyin
$userData->isAdmin = true;
$userData->groupIds = ['group-1', 'group-2'];

// API anahtarınızla SSO nesnesini oluşturun
$sso = FastCommentsSSO::createSecure('your-api-key', $userData);

// FastComments'a iletmek için token'ı al
$token = $sso->prepareToSend();
```