### 간단한 SSO

간단한 SSO는 사용하기 쉽지만 Secure SSO보다 보안이 낮습니다:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SimpleSSOUserData;

// 사용자 데이터 생성
$userData = SimpleSSOUserData::create(
    'username123',
    'user@example.com',
    'https://example.com/avatar.png'
);

// URL 기반 로그인/로그아웃 사용
$sso = FastCommentsSSO::createWithUrls(
    null,
    $userData,
    'https://example.com/login',
    'https://example.com/logout'
);

// 또는 콜백 사용
$sso = FastCommentsSSO::createWithCallbacks(
    $userData,
    function($url) { /* 로그인 처리 */ },
    function($url) { /* 로그아웃 처리 */ }
);

// FastComments에 전달할 토큰 가져오기
$token = $sso->prepareToSend();
```

### 보안 SSO

Secure SSO는 HMAC 검증으로 향상된 보안을 제공합니다:

```php
use FastComments\SSO\FastCommentsSSO;
use FastComments\SSO\SecureSSOUserData;

// 사용자 데이터 생성
$userData = SecureSSOUserData::create(
    'user-123',
    'user@example.com',
    'username123',
    'https://example.com/avatar.png'
);

// 필요한 경우 선택적 데이터 추가
$userData->isAdmin = true;
$userData->groupIds = ['group-1', 'group-2'];

// API 키로 SSO 객체 생성
$sso = FastCommentsSSO::createSecure('your-api-key', $userData);

// FastComments에 전달할 토큰 가져오기
$token = $sso->prepareToSend();
```