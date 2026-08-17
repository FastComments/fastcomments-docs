## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| urlId | string | query | כן |  |

## תגובה

מחזיר: [`GetPageByURLIdAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPageByURLIdAPIResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getPageByURLId'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// הגדרת הרשאת מפתח API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// בטל את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם נדרש
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם, העבר את הלקוח שלך שמממש `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // מחרוזת
$url_id = 'url_id_example'; // מחרוזת


try {
    $result = $apiInstance->getPageByURLId($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getPageByURLId: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]