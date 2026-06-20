## פרמטרים

| שם | Type | Location | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | לא |  |

## תגובה

מחזיר: [`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/BulkCreateHashTagsResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-addHashTagsBulk'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// הגדר הרשאת מפתח ה-API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// בטל את ההערה למטה כדי להגדיר קידומת (למשל Bearer) עבור מפתח ה-API, אם נדרש
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי; `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$bulk_create_hash_tags_body = new \FastComments\Client\Model\BulkCreateHashTagsBody(); // \FastComments\Client\Model\BulkCreateHashTagsBody

try {
    $result = $apiInstance->addHashTagsBulk($tenant_id, $bulk_create_hash_tags_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->addHashTagsBulk: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---