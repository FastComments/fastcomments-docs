## 參數

| 名稱 | 型別 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| isFlagged | boolean | query | Yes |  |
| sso | string | query | No |  |

## 回應

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## 範例

[inline-code-attrs-start title = 'flagCommentPublic 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自訂的 HTTP 客戶端，傳入實作 `GuzzleHttp\ClientInterface` 的客戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // 字串
$comment_id = 'comment_id_example'; // 字串
$is_flagged = True; // 布林
$sso = 'sso_example'; // 字串


try {
    $result = $apiInstance->flagCommentPublic($tenant_id, $comment_id, $is_flagged, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->flagCommentPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]