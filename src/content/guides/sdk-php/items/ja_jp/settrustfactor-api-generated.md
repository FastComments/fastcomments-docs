## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| userId | string | query | いいえ |  |
| trustFactor | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SetUserTrustFactorResponse.php)

## 例

[inline-code-attrs-start title = 'setTrustFactor の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // カスタムの HTTP クライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これは任意です。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);
$user_id = 'user_id_example'; // 文字列
$trust_factor = 'trust_factor_example'; // 文字列
$sso = 'sso_example'; // 文字列

try {
    $result = $apiInstance->setTrustFactor($user_id, $trust_factor, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->setTrustFactor: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]