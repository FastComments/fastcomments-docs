## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |

## תגובה

מחזיר: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/FlagCommentPublic200Response.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-updateEmailTemplate'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// הגדר הרשאת מפתח ה-API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// הסר את ההערה למטה כדי להגדיר קידומת (למשל Bearer) עבור מפתח ה-API, במידת הצורך
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם אישית, העבר את הלקוח שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, ייעשה שימוש ב-`GuzzleHttp\Client` כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$update_email_template_body = new \FastComments\Client\Model\UpdateEmailTemplateBody(); // \FastComments\Client\Model\UpdateEmailTemplateBody

try {
    $result = $apiInstance->updateEmailTemplate($tenant_id, $id, $update_email_template_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->updateEmailTemplate: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---