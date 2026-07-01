## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |

## תגובה

מחזיר: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/FlagCommentResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמת unFlagComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// // הגדר אישור מפתח API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// // הסר את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח API, אם נדרש
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // // אם ברצונך להשתמש בלקוח http מותאם, העבר את הלקוח שלך שמיישם `GuzzleHttp\ClientInterface`.
    // // זה אופציונלי, `GuzzleHttp\Client` יישמש כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // מחרוזת
$id = 'id_example'; // מחרוזת
$options = [
    'user_id' => 'user_id_example', // מחרוזת
    'anon_user_id' => 'anon_user_id_example', // מחרוזת
];


try {
    $result = $apiInstance->unFlagComment($tenant_id, $id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->unFlagComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]