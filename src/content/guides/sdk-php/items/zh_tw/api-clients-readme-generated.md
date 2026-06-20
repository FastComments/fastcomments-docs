此 SDK 提供三個 API 用戶端類別：

- **`DefaultApi`** — 用於伺服器端且以 API 金鑰驗證的方法。將 API 金鑰設定為如 [開始使用](#getting-started-readme-generated) 所示。
- **`PublicApi`** — 不需 API 金鑰的公開方法，可安全從瀏覽器與行動應用程式呼叫。
- **`ModerationApi`** — 供管理員儀表板使用的方法：列出、計數、搜尋、記錄與匯出留言；審核操作（移除/還原、標記、設定審查/垃圾郵件/核准狀態、投票、重新開啟/關閉討論串）；禁令（禁止留言、撤銷、預先禁令摘要、禁令狀態與偏好、被禁用使用者數量）；以及徽章與信任（授予/移除徽章、手動徽章、取得/設定信任因子、使用者內部檔案）。每個 `ModerationApi` 方法都接受一個 `$sso` 參數，以透過 SSO 驗證執行操作的管理員。

### 使用 PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// 公開方法不需要 API 金鑰。
$apiInstance = new FastComments\Client\Api\PublicApi(
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // 字串
$url_id = 'url_id_example'; // 字串

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
$sso = 'sso_example'; // 字串 - 用來透過 SSO 驗證管理員的 SSO 載荷

try {
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```