## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| userId | string | query | לא |  |
| badgeId | string | query | לא |  |
| type | number | query | לא |  |
| displayedOnComments | boolean | query | לא |  |
| limit | number | query | לא |  |
| skip | number | query | לא |  |

## תשובה

מחזיר: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIGetUserBadgesResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getUserBadges'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// הגדר הרשאת מפתח API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// הסר את ההערה למטה כדי להגדיר קידומת (לדוגמה Bearer) למפתח ה-API, אם נדרש
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם, העבר את הלקוח שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי; `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // מחרוזת
$user_id = 'user_id_example'; // מחרוזת
$badge_id = 'badge_id_example'; // מחרוזת
$type = 3.4; // מספר עשרוני
$displayed_on_comments = True; // בוליאני
$limit = 3.4; // מספר עשרוני
$skip = 3.4; // מספר עשרוני

try {
    $result = $apiInstance->getUserBadges($tenant_id, $user_id, $badge_id, $type, $displayed_on_comments, $limit, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getUserBadges: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---