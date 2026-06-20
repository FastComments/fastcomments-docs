## パラメーター

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserTrustFactorResponse.php)

## 例

[inline-code-attrs-start title = 'getTrustFactor の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // カスタムの HTTP クライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これは省略可能で、デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);
$user_id = 'user_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getTrustFactor($user_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getTrustFactor: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]