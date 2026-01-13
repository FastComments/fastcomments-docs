## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| userId | string | query | לא |  |
| urlId | string | query | לא |  |
| fromCommentId | string | query | לא |  |
| viewed | boolean | query | לא |  |
| type | string | query | לא |  |

## תגובה

מחזיר: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetNotificationCount200Response.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getNotificationCount'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// הגדר הרשאת מפתח API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// הסר את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם יש צורך
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם אישית, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // מחרוזת
$user_id = 'user_id_example'; // מחרוזת
$url_id = 'url_id_example'; // מחרוזת
$from_comment_id = 'from_comment_id_example'; // מחרוזת
$viewed = True; // בוליאני
$type = 'type_example'; // מחרוזת

try {
    $result = $apiInstance->getNotificationCount($tenant_id, $user_id, $url_id, $from_comment_id, $viewed, $type);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getNotificationCount: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]