## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| urlId | string | query | לא |  |
| userId | string | query | לא |  |
| startDate | string | query | לא |  |
| questionId | string | query | לא |  |
| questionIds | string | query | לא |  |
| skip | number | query | לא |  |

## תגובה

מחזיר: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetQuestionResultsResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getQuestionResults'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// קביעת הרשאת מפתח API: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// בטלו את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח API, אם נדרש


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // אם ברצונך להשתמש בלקוח HTTP מותאם, העבר את הלקוח שלך הממומש `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
// מחרוזת
$options = [
    'url_id' => 'url_id_example', // string
    // מחרוזת
    'user_id' => 'user_id_example', // string
    // מחרוזת
    'start_date' => 'start_date_example', // string
    // מחרוזת
    'question_id' => 'question_id_example', // string
    // מחרוזת
    'question_ids' => 'question_ids_example', // string
    // מחרוזת
    'skip' => 3.4, // float
    // מספר עשרוני
];


try {
    $result = $apiInstance->getQuestionResults($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getQuestionResults: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---