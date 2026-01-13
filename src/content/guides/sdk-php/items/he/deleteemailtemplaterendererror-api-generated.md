## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |
| errorId | string | path | כן |  |

## תגובה

מחזיר: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/FlagCommentPublic200Response.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-deleteEmailTemplateRenderError'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// תצורת אישור מפתח API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// הסר את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח ה-API, אם צריך
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי; ישתמש ב-`GuzzleHttp\Client` כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // מחרוזת
$id = 'id_example'; // מחרוזת
$error_id = 'error_id_example'; // מחרוזת

try {
    $result = $apiInstance->deleteEmailTemplateRenderError($tenant_id, $id, $error_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteEmailTemplateRenderError: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---