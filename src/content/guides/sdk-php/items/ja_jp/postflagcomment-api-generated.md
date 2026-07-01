## Parameters

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| commentId | string | path | はい |  |
| broadcastId | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## Response

返却: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Example

[inline-code-attrs-start title = 'postFlagComment 例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // カスタム HTTP クライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装したクライアントを渡してください。
    // これはオプションで、デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$options = [
    'broadcast_id' => 'broadcast_id_example', // string
    'sso' => 'sso_example', // string
];


try {
    $result = $apiInstance->postFlagComment($tenant_id, $comment_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postFlagComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]