## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |
| userId | string | query | לא |  |

## תגובה

מחזיר: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/FlagCommentPublic200Response.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-updateNotification'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// הגדר את אישור מפתח ה-API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// בטל את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם צריך
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם אישית, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$update_notification_body = new \FastComments\Client\Model\UpdateNotificationBody(); // \FastComments\Client\Model\UpdateNotificationBody
$user_id = 'user_id_example'; // string

try {
    $result = $apiInstance->updateNotification($tenant_id, $id, $update_notification_body, $user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->updateNotification: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]