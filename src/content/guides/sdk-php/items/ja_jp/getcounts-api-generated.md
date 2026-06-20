## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetBannedUsersCountResponse.php)

## 例

[inline-code-attrs-start title = 'getCounts の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // カスタムの HTTP クライアントを使用する場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これは任意です。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getCounts($sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCounts: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]