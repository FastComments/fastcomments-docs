## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | クエリ | はい |  |
| id | string | パス | はい |  |
| updateComments | boolean | クエリ | いいえ |  |

## レスポンス

戻り値: [`PutSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PutSSOUserAPIResponse.php)

## 例

[inline-code-attrs-start title = 'putSSOUser の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// APIキー認証を設定: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 必要に応じて下行のコメントを外してAPIキーの接頭辞（例: Bearer）を設定してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // カスタムHTTPクライアントを使用する場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これは任意です。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string (文字列)
$id = 'id_example'; // string (文字列)
$update_apisso_user_data = new \FastComments\Client\Model\UpdateAPISSOUserData(); // \FastComments\Client\Model\UpdateAPISSOUserData (UpdateAPISSOUserData オブジェクト)
$update_comments = True; // bool (真偽値)

try {
    $result = $apiInstance->putSSOUser($tenant_id, $id, $update_apisso_user_data, $update_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->putSSOUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]