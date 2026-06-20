---
## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| updateComments | string | query | No |  |

## תגובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-replaceTenantUser'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// הגדר אימות באמצעות מפתח API: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// הסר את ההערה למטה כדי להגדיר קידומת (למשל Bearer) עבור מפתח ה-API, במידת הצורך
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // אם ברצונך להשתמש בלקוח HTTP מותאם אישית, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // זה אופציונלי, ישתמש ב-`GuzzleHttp\Client` כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // מחרוזת
$id = 'id_example'; // מחרוזת
$replace_tenant_user_body = new \FastComments\Client\Model\ReplaceTenantUserBody(); // \FastComments\Client\Model\ReplaceTenantUserBody
$update_comments = 'update_comments_example'; // מחרוזת

try {
    $result = $apiInstance->replaceTenantUser($tenant_id, $id, $replace_tenant_user_body, $update_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->replaceTenantUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---