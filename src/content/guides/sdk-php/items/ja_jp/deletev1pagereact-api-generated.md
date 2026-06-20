## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | パス | はい |  |
| urlId | string | クエリ | はい |  |

## レスポンス

戻り値: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateV1PageReact.php)

## 例

[inline-code-attrs-start title = 'deleteV1PageReact の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // カスタムのHTTPクライアントを使用する場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これはオプションです。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string

try {
    $result = $apiInstance->deleteV1PageReact($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->deleteV1PageReact: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]