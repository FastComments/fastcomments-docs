## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`APIModerateGetUserBanPreferencesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIModerateGetUserBanPreferencesResponse.php)

## 例

[inline-code-attrs-start title = 'getUserBanPreference の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // カスタムHTTPクライアントを使用する場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これは任意です。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string 型

try {
    $result = $apiInstance->getUserBanPreference($sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getUserBanPreference: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]