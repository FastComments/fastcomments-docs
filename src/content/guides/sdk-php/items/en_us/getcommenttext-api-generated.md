## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| editKey | string | query | No |  |
| sso | string | query | No |  |

## Response

Returns: [`GetCommentText200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentText200Response.php)

## Example

[inline-code-attrs-start title = 'getCommentText Example'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // If you want to use a custom HTTP client, pass your client that implements `GuzzleHttp\ClientInterface`.
    // This is optional; `GuzzleHttp\Client` will be used as the default.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$edit_key = 'edit_key_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getCommentText($tenant_id, $comment_id, $edit_key, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentText: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]