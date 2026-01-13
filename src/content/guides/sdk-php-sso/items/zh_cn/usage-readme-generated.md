### 简单 SSO

简单 SSO 使用起来很直接，但比安全 SSO 提供的安全性要低：

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SimpleSSOUserData;

// 创建用户数据
$userData = SimpleSSOUserData::create(
    'username123',
    'user@example.com',
    'https://example.com/avatar.png'
);

// 使用基于 URL 的登录/登出
$sso = FastCommentsSSO::createWithUrls(
    null,
    $userData,
    'https://example.com/login',
    'https://example.com/logout'
);

// 或使用回调
$sso = FastCommentsSSO::createWithCallbacks(
    $userData,
    function($url) { /* 处理登录 */ },
    function($url) { /* 处理登出 */ }
);

// 获取要传递给 FastComments 的令牌
$token = $sso->prepareToSend();
```

### 安全 SSO

安全 SSO 通过 HMAC 验证提供增强的安全性：

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SecureSSOUserData;

// 创建用户数据
$userData = SecureSSOUserData::create(
    'user-123',
    'user@example.com',
    'username123',
    'https://example.com/avatar.png'
);

// 如有需要，添加可选数据
$userData->isAdmin = true;
$userData->groupIds = ['group-1', 'group-2'];

// 使用您的 API 密钥创建 SSO 对象
$sso = FastCommentsSSO::createSecure('your-api-key', $userData);

// 获取要传递给 FastComments 的令牌
$token = $sso->prepareToSend();
```