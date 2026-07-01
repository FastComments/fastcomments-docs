## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## תשובה

מחזיר: [`CreateEmailTemplateResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateEmailTemplateResponse.php)

## דוגמה

[inline-code-attrs-start title = 'createEmailTemplate דוגמה'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// הגדר רישיון מפתח API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// בטל את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח API, אם צריך
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם, העבר את הלקוח שלך שמממש `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש ברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // מחרוזת
$create_email_template_body = new \FastComments\Client\Model\CreateEmailTemplateBody(); // \FastComments\Client\Model\CreateEmailTemplateBody


try {
    $result = $apiInstance->createEmailTemplate($tenant_id, $create_email_template_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createEmailTemplate: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]