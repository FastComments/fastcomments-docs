## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| skip | integer | query | לא |  |

## תגובה

מחזיר: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetSSOUsersResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getSSOUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// הגדר הרשאת מפתח API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// בטל את ההערה למטה כדי להגדיר קידומת (למשל Bearer) עבור מפתח ה-API, אם יש צורך
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם אישית, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי; כברירת מחדל ישמש `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // מחרוזת
$skip = 56; // מספר שלם

try {
    $result = $apiInstance->getSSOUsers($tenant_id, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getSSOUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]