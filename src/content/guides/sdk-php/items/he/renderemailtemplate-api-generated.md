## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| locale | string | query | לא |  |

## תגובה

מחזיר: [`RenderEmailTemplate200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/RenderEmailTemplate200Response.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-renderEmailTemplate'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// הסר את ההערה למטה כדי להגדיר קידומת (למשל Bearer) עבור מפתח ה-API, אם יש צורך
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // אם ברצונך להשתמש בלקוח HTTP מותאם אישית, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$render_email_template_body = new \FastComments\Client\Model\RenderEmailTemplateBody(); // \FastComments\Client\Model\RenderEmailTemplateBody
$locale = 'locale_example'; // string

try {
    $result = $apiInstance->renderEmailTemplate($tenant_id, $render_email_template_body, $locale);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->renderEmailTemplate: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---