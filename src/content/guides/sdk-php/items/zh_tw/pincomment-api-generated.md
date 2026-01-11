## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | 路徑 | 是 |  |
| commentId | string | 路徑 | 是 |  |
| broadcastId | string | 查詢 | 是 |  |
| sso | string | 查詢 | 否 |  |

## 回應

回傳: [`PinComment200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PinComment200Response.php)

## 範例

[inline-code-attrs-start title = 'pinComment 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想要使用自訂的 HTTP 用戶端，請傳入實作了 `GuzzleHttp\ClientInterface` 的用戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // 字串
$comment_id = 'comment_id_example'; // 字串
$broadcast_id = 'broadcast_id_example'; // 字串
$sso = 'sso_example'; // 字串

try {
    $result = $apiInstance->pinComment($tenant_id, $comment_id, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->pinComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---