### シンプルなSSO

シンプルなSSOは使いやすいですが、セキュアなSSOよりもセキュリティが低くなります：

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SimpleSSOUserData;

// ユーザーデータを作成
$userData = SimpleSSOUserData::create(
    'username123',
    'user@example.com',
    'https://example.com/avatar.png'
);

// URLベースのログイン/ログアウトを使用
$sso = FastCommentsSSO::createWithUrls(
    null,
    $userData,
    'https://example.com/login',
    'https://example.com/logout'
);

// またはコールバックを使用
$sso = FastCommentsSSO::createWithCallbacks(
    $userData,
    function($url) { /* ログインを処理 */ },
    function($url) { /* ログアウトを処理 */ }
);

// FastCommentsに渡すトークンを取得
$token = $sso->prepareToSend();
```

### セキュアなSSO

セキュアなSSOはHMAC検証によって高度なセキュリティを提供します：

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SecureSSOUserData;

// ユーザーデータを作成
$userData = SecureSSOUserData::create(
    'user-123',
    'user@example.com',
    'username123',
    'https://example.com/avatar.png'
);

// 必要に応じてオプションのデータを追加
$userData->isAdmin = true;
$userData->groupIds = ['group-1', 'group-2'];

// APIキーを使用してSSOオブジェクトを作成
$sso = FastCommentsSSO::createSecure('your-api-key', $userData);

// FastCommentsに渡すトークンを取得
$token = $sso->prepareToSend();
```