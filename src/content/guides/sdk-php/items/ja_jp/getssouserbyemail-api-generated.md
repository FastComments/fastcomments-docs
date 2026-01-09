## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| email | string | path | はい |  |

## レスポンス

戻り値: [`GetSSOUserByEmailAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetSSOUserByEmailAPIResponse.php)

## 例

[inline-code-attrs-start title = 'getSSOUserByEmail の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// APIキー認証を設定します: api_key
// 必要な場合は下の行のコメントアウトを外して、APIキー用のプレフィックス（例：Bearer）を設定してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // カスタムのHTTPクライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これはオプションです。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$email = 'email_example'; // string

try {
    $result = $apiInstance->getSSOUserByEmail($tenant_id, $email);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getSSOUserByEmail: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]