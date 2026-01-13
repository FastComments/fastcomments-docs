## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| skip | number | query | לא |  |

## תגובה

מחזיר: [`GetEmailTemplates200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetEmailTemplates200Response.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getEmailTemplates'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// הגדר הרשאת מפתח API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// בטל את ההערה מתחת כדי להגדיר קידומת (למשל Bearer) עבור מפתח ה-API, אם נדרש
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // מחרוזת
$skip = 3.4; // מספר עם נקודה צפה

try {
    $result = $apiInstance->getEmailTemplates($tenant_id, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getEmailTemplates: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---