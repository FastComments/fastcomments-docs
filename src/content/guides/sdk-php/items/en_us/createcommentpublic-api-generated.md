## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| broadcastId | string | query | Yes |  |
| sessionId | string | query | No |  |
| sso | string | query | No |  |

## Response

Returns: [`CreateCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateCommentPublic200Response.php)

## Example

[inline-code-attrs-start title = 'createCommentPublic Example'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // If you want to use a custom HTTP client, pass a client that implements `GuzzleHttp\ClientInterface`.
    // This is optional; `GuzzleHttp\Client` will be used by default.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$broadcast_id = 'broadcast_id_example'; // string
$comment_data = new \FastComments\Client\Model\CommentData(); // \FastComments\Client\Model\CommentData
$session_id = 'session_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->createCommentPublic($tenant_id, $url_id, $broadcast_id, $comment_data, $session_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->createCommentPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]