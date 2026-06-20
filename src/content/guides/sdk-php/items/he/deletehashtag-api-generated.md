## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tag | string | נתיב | כן |  |
| tenantId | string | שאילתה | לא |  |

## תגובה

מחזיר: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-deleteHashTag'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// הגדר הרשאת מפתח API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// בטל את ההערה למטה כדי להגדיר קידומת (לדוגמה Bearer) עבור מפתח ה-API, אם נדרש
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // אם ברצונך להשתמש ב-client HTTP מותאם, העבר את ה-client שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);
$tag = 'tag_example'; // מחרוזת
$tenant_id = 'tenant_id_example'; // מחרוזת
$delete_hash_tag_request_body = new \FastComments\Client\Model\DeleteHashTagRequestBody(); // \FastComments\Client\Model\DeleteHashTagRequestBody

try {
    $result = $apiInstance->deleteHashTag($tag, $tenant_id, $delete_hash_tag_request_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteHashTag: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]