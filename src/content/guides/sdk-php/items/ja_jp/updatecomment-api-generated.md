## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|------|-------------|
| tenantId | string | query | はい |  |
| id | string | path | はい |  |
| contextUserId | string | query | いいえ |  |
| doSpamCheck | boolean | query | いいえ |  |
| isLive | boolean | query | いいえ |  |

## 応答

戻り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## 例

[inline-code-attrs-start title = 'updateComment の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// APIキー認証を設定: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 必要に応じて、以下のコメントを外して API キーのプレフィックス (例: Bearer) を設定してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // カスタム HTTP クライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装したクライアントを渡してください。
    // これはオプションです。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$updatable_comment_params = new \FastComments\Client\Model\UpdatableCommentParams(); // \FastComments\Client\Model\UpdatableCommentParams
$options = [
    'context_user_id' => 'context_user_id_example', // string
    'do_spam_check' => True, // bool
    'is_live' => True, // bool
];


try {
    $result = $apiInstance->updateComment($tenant_id, $id, $updatable_comment_params, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->updateComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]