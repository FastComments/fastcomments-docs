## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| questionId | string | query | No |  |
| questionIds | array | query | No |  |
| urlId | string | query | No |  |
| startDate | string | query | No |  |
| forceRecalculate | boolean | query | No |  |
| minValue | number | query | No |  |
| maxValue | number | query | No |  |
| limit | number | query | No |  |

## תגובה

מחזיר: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CombineQuestionResultsWithCommentsResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה של combineCommentsWithQuestionResults'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// הגדר הרשאת מפתח API: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// בטל את ההערה למטה כדי להגדיר קידומת (לדוגמה Bearer) למפתח ה-API, אם נדרש
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // אם ברצונך להשתמש בלקוח HTTP מותאם, העבר את הלקוח שלך שמממש `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'question_id' => 'question_id_example', // string
    'question_ids' => array('question_ids_example'), // string[]
    'url_id' => 'url_id_example', // string
    'start_date' => new \DateTime('2013-10-20T19:20:30+01:00'), // \DateTime
    'force_recalculate' => True, // bool
    'min_value' => 3.4, // float
    'max_value' => 3.4, // float
    'limit' => 3.4, // float
];


try {
    $result = $apiInstance->combineCommentsWithQuestionResults($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->combineCommentsWithQuestionResults: ', $e->getMessage(), PHP_EOL;
    // חריגה בזמן קריאה ל-DefaultApi->combineCommentsWithQuestionResults: 
}
[inline-code-end]