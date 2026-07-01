The SDK는 세 개의 API 클라이언트 클래스를 제공합니다:

- **`DefaultApi`** - API 키 인증 메서드이며 서버‑사이드 용으로 사용됩니다. API 키는 [Getting Started](#getting-started-readme-generated)와 같이 구성합니다.
- **`PublicApi`** - API 키가 필요 없는 공개 메서드이며 브라우저와 모바일 앱에서 안전하게 호출할 수 있습니다.
- **`ModerationApi`** - 실시간 및 빠른 검토 API의 광범위한 스위트입니다. 모든 `ModerationApi` 메서드는 `$sso` 파라미터를 받아 SSO 또는 FastComments.com 세션 쿠키를 통해 인증할 수 있습니다.

### PublicApi 사용하기

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// 공개 메서드는 API 키가 필요하지 않습니다.
$apiInstance = new FastComments\Client\Api\PublicApi(
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string

try {
    $result = $apiInstance->getCommentsPublic($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
```

### ModerationApi 사용하기

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - Moderator를 인증하는 SSO 페이로드

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```