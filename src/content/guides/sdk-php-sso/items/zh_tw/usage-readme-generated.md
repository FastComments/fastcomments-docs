### 簡易 SSO

簡易 SSO 使用起來很簡單，但安全性低於安全 SSO：

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SimpleSSOUserData;

// 建立使用者資料
$userData = SimpleSSOUserData::create(
    'username123',
    'user@example.com',
    'https://example.com/avatar.png'
);

// 使用基於 URL 的登入/登出
$sso = FastCommentsSSO::createWithUrls(
    null,
    $userData,
    'https://example.com/login',
    'https://example.com/logout'
);

// 或使用回呼函式
$sso = FastCommentsSSO::createWithCallbacks(
    $userData,
    function($url) { /* 處理登入 */ },
    function($url) { /* 處理登出 */ }
);

// 取得要傳遞給 FastComments 的權杖
$token = $sso->prepareToSend();
```

### 安全 SSO

安全 SSO 透過 HMAC 驗證提供增強的安全性：

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SecureSSOUserData;

// 建立使用者資料
$userData = SecureSSOUserData::create(
    'user-123',
    'user@example.com',
    'username123',
    'https://example.com/avatar.png'
);

// 視需要新增選填資料
$userData->isAdmin = true;
$userData->groupIds = ['group-1', 'group-2'];

// 使用您的 API 金鑰建立 SSO 物件
$sso = FastCommentsSSO::createSecure('your-api-key', $userData);

// 取得要傳遞給 FastComments 的權杖
$token = $sso->prepareToSend();
```