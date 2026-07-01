## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| redirectURL | string | query | No |  |

## תגובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמת sendLoginLink'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// קבעו הרשאת מפתח API: api_key
// בטלו הערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם נדרש
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // אם ברצונכם להשתמש בלקוח HTTP מותאם, העבירו את הלקוח שלכם המממש `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // מחרוזת
$id = 'id_example'; // מחרוזת
$redirect_url = 'redirect_url_example'; // מחרוזת


try {
    $result = $apiInstance->sendLoginLink($tenant_id, $id, $redirect_url);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->sendLoginLink: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]