---
## パラメーター

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| commentId | string | query | はい |  |
| direction | string | query | はい |  |
| userId | string | query | いいえ |  |
| anonUserId | string | query | いいえ |  |

## レスポンス

戻り値: [`VoteResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/VoteResponse.php)

## 例

[inline-code-attrs-start title = 'createVoteの例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// APIキー認証の設定: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 必要に応じて、APIキーにプレフィックス（例: Bearer）を設定する場合は下の行のコメントを外してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // カスタムのHTTPクライアントを使用する場合は、`GuzzleHttp\ClientInterface` を実装したクライアントを渡してください。
    // これは任意です。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$direction = 'direction_example'; // string
$user_id = 'user_id_example'; // string
$anon_user_id = 'anon_user_id_example'; // string

try {
    $result = $apiInstance->createVote($tenant_id, $comment_id, $direction, $user_id, $anon_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---