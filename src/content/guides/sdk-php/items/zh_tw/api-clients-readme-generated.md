The SDK 提供三個 API 客戶端類別：

- **`DefaultApi`** - 需要 API 金鑰認證的伺服器端方法。 如 [Getting Started](#getting-started-readme-generated) 所示配置 API 金鑰。
- **`PublicApi`** - 不需要 API 金鑰的公共方法，可安全從瀏覽器和行動應用程式呼叫。
- **`ModerationApi`** - 提供廣泛的即時與快速審核 API。每個 `ModerationApi` 方法都接受 `$sso` 參數，並可透過 SSO 或 FastComments.com 會話 Cookie 進行驗證。

### 使用 PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// 公共方法不需要 API 金鑰。
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
$sso = 'sso_example'; // string - 驗證審核者的 SSO 負載

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```