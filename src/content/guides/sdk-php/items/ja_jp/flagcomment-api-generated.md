## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |
| userId | string | query | いいえ |  |
| anonUserId | string | query | いいえ |  |

## レスポンス

戻り値: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/FlagCommentResponse.php)

## 例

[inline-code-attrs-start title = 'flagComment の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// APIキー認証を設定: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 必要に応じて API キーのプレフィックス（例：Bearer）を設定するには、以下のコメントを外してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // カスタムの HTTP クライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これはオプションで、デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // 文字列
$id = 'id_example'; // 文字列
$user_id = 'user_id_example'; // 文字列
$anon_user_id = 'anon_user_id_example'; // 文字列

try {
    $result = $apiInstance->flagComment($tenant_id, $id, $user_id, $anon_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->flagComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]