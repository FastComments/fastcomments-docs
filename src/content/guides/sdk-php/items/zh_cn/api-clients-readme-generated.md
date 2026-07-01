The SDK 提供了三个 API 客户端类：

- **`DefaultApi`** - 用于服务器端的 API 密钥认证方法。按照 [Getting Started](#getting-started-readme-generated) 中的示例配置 API 密钥。
- **`PublicApi`** - 不需要 API 密钥的公共方法，可安全地在浏览器和移动应用中调用。
- **`ModerationApi`** - 一个包括实时和快速审查功能的完整套件。每个 `ModerationApi` 方法都接受 `$sso` 参数，并可通过 SSO 或 FastComments.com 会话 Cookie 进行身份验证。

### 使用 PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// 公共方法不需要 API 密钥。
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

### 使用 ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - 用于验证审查员的 SSO 负载

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```