## פרמטרים

| שם | סוג | מיקום | הכרחי | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## תגובה

מחזיר: [`GetModeratorResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetModeratorResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getModerator'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// // הגדר הרשאת מפתח API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// // בטל את ההערה למטה כדי להגדיר קידומת (לדוגמה Bearer) למפתח API, אם נדרש
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // // אם אתה רוצה להשתמש בלקוח http מותאם אישית, העבר את הלקוח שלך שמיישם `GuzzleHttp\ClientInterface`.
    // // זה אפשרי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // מחרוזת
$id = 'id_example'; // מחרוזת


try {
    $result = $apiInstance->getModerator($tenant_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getModerator: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]