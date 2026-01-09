## 參數

| 名稱 | 型別 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| commentId | string | path | 是 |  |
| broadcastId | string | query | 是 |  |
| editKey | string | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`DeleteCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteCommentPublic200Response.php)

## 範例

[inline-code-attrs-start title = 'deleteCommentPublic 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 若要使用自訂的 HTTP 用戶端，請傳入實作 `GuzzleHttp\ClientInterface` 的用戶端。
    // 這是選用的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$broadcast_id = 'broadcast_id_example'; // string
$edit_key = 'edit_key_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->deleteCommentPublic($tenant_id, $comment_id, $broadcast_id, $edit_key, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->deleteCommentPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]