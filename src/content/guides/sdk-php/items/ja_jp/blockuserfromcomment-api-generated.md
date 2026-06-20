## パラメーター

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |
| userId | string | query | いいえ |  |
| anonUserId | string | query | いいえ |  |

## レスポンス

返却値: [`BlockSuccess`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BlockSuccess.php)

## 例

[inline-code-attrs-start title = 'blockUserFromComment の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// APIキー認証を設定: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 必要に応じて、APIキーに接頭辞（例: Bearer）を設定するため、以下のコメントを解除してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // カスタムHTTPクライアントを使用する場合は、`GuzzleHttp\ClientInterface` を実装したクライアントを渡してください。
    // これはオプションです。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // 文字列
$id = 'id_example'; // 文字列
$block_from_comment_params = new \FastComments\Client\Model\BlockFromCommentParams(); // \FastComments\Client\Model\BlockFromCommentParams
$user_id = 'user_id_example'; // 文字列
$anon_user_id = 'anon_user_id_example'; // 文字列

try {
    $result = $apiInstance->blockUserFromComment($tenant_id, $id, $block_from_comment_params, $user_id, $anon_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->blockUserFromComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---