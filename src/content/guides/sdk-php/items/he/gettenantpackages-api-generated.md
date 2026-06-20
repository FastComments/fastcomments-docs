## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| skip | number | query | לא |  |

## תגובה

מחזיר: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenantPackagesResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getTenantPackages'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// הגדר אימות מפתח API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// אם יש צורך, הסר את ההערה למטה כדי להגדיר קידומת (לדוגמה Bearer) עבור מפתח ה-API
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם אישית, העבר לקוח שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$skip = 3.4; // float

try {
    $result = $apiInstance->getTenantPackages($tenant_id, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTenantPackages: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]