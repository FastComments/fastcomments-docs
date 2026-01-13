## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| commentId | string | query | כן |  |
| direction | string | query | כן |  |
| userId | string | query | לא |  |
| anonUserId | string | query | לא |  |

## תגובה

מחזיר: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/VoteComment200Response.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-createVote'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // מחרוזת
$comment_id = 'comment_id_example'; // מחרוזת
$direction = 'direction_example'; // מחרוזת
$user_id = 'user_id_example'; // מחרוזת
$anon_user_id = 'anon_user_id_example'; // מחרוזת

try {
    $result = $apiInstance->createVote($tenant_id, $comment_id, $direction, $user_id, $anon_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]