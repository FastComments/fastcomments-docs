## パラメータ

| 名前 | 型 | ロケーション | 必須 | 説明 |
|------|------|----------|------|------|
| tenantId | string | query | はい |  |
| urlIdWS | string | query | はい |  |
| userIds | string | query | はい |  |

## 応答

返り値: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserPresenceStatusesResponse.php)

## 例

[inline-code-attrs-start title = 'getUserPresenceStatuses の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // カスタム HTTP クライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これはオプションです。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // 文字列
$url_id_ws = 'url_id_ws_example'; // 文字列
$user_ids = 'user_ids_example'; // 文字列


try {
    $result = $apiInstance->getUserPresenceStatuses($tenant_id, $url_id_ws, $user_ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUserPresenceStatuses: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]