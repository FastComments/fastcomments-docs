## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## תגובה

מחזיר: [`GetPagesAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPagesAPIResponse.php)

## דוגמת getPages

[inline-code-attrs-start title = 'דוגמת getPages'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// הגדרת הרשאת מפתח API: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// בטל את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם נדרש
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // אם ברצונך להשתמש בלקוח HTTP מותאם, העבר את הלקוח שלך שמיישם `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string


try {
    $result = $apiInstance->getPages($tenant_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getPages: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]