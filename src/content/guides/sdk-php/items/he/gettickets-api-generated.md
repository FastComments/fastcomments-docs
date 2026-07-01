## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| state | number | query | No |  |
| skip | number | query | No |  |
| limit | number | query | No |  |

## תשובה

מחזיר: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTicketsResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getTickets'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// קבע תצורת הרשאת מפתח API: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// בטל את ההערה למטה כדי להגדיר קידומת (לדוגמה Bearer) למפתח ה-API, במידה ונדרש
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // אם ברצונך להשתמש ב‑client HTTP מותאם, העבר את ה‑client שלך שמממש `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // מחרוזת
$options = [
    'user_id' => 'user_id_example', // מחרוזת
    'state' => 3.4, // מספר ממשי
    'skip' => 3.4, // מספר ממשי
    'limit' => 3.4, // מספר ממשי
];


try {
    $result = $apiInstance->getTickets($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTickets: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]