## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgesUserId | string | query | いいえ |  |
| commentId | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserManualBadgesResponse.php)

## 例

[inline-code-attrs-start title = 'getManualBadgesForUser の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // カスタムの HTTP クライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これは任意で、デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);
$badges_user_id = 'badges_user_id_example'; // string
$comment_id = 'comment_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getManualBadgesForUser($badges_user_id, $comment_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getManualBadgesForUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]