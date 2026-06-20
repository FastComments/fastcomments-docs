## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| meta | string | query | No |  |
| skip | number | query | No |  |

## תגובה

מחזיר: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenantsResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getTenants'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// הגדר אישור מפתח API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// הסר את ההערה שלמטה כדי להגדיר קידומת (למשל Bearer) עבור מפתח ה-API, אם נדרש
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם אישית, העבר את הלקוח שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי; ברירת המחדל היא `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // מחרוזת
$meta = 'meta_example'; // מחרוזת
$skip = 3.4; // מספר עשרוני

try {
    $result = $apiInstance->getTenants($tenant_id, $meta, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTenants: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---