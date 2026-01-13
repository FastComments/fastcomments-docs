## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## תגובה

מחזיר: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/FlagCommentPublic200Response.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-updateQuestionResult'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// הגדר הרשאת מפתח API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם, העבר את הלקוח שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$update_question_result_body = new \FastComments\Client\Model\UpdateQuestionResultBody(); // \FastComments\Client\Model\UpdateQuestionResultBody

try {
    $result = $apiInstance->updateQuestionResult($tenant_id, $id, $update_question_result_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->updateQuestionResult: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]