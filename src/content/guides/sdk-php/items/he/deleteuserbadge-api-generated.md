## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |

## Response

מחזיר: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptySuccessResponse.php)

## Example

[inline-code-attrs-start title = 'deleteUserBadge דוגמה'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// // הגדר את אימות מפתח ה‑API: api_key
// // בטל את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה‑API, אם נדרש
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // // אם ברצונך להשתמש ב‑client HTTP מותאם אישית, העבר את ה‑client שלך שמממש `GuzzleHttp\ClientInterface`.
    // // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // מחרוזת
$id = 'id_example'; // מחרוזת


try {
    $result = $apiInstance->deleteUserBadge($tenant_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteUserBadge: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]