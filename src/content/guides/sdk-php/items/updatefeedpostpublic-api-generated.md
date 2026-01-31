## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| postId | string | path | Yes |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Response

Returns: [`CreateFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateFeedPostPublic200Response.php)

## Example

[inline-code-attrs-start title = 'updateFeedPostPublic Example'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$post_id = 'post_id_example'; // string
$update_feed_post_params = new \FastComments\Client\Model\UpdateFeedPostParams(); // \FastComments\Client\Model\UpdateFeedPostParams
$broadcast_id = 'broadcast_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->updateFeedPostPublic($tenant_id, $post_id, $update_feed_post_params, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->updateFeedPostPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]
