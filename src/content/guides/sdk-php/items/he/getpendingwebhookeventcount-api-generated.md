## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | query | No |  |
| externalId | string | query | No |  |
| eventType | string | query | No |  |
| type | string | query | No |  |
| domain | string | query | No |  |
| attemptCountGT | number | query | No |  |

## תגובה

מחזירה: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPendingWebhookEventCountResponse.php)

## דוגמה

[inline-code-attrs-start title = 'getPendingWebhookEventCount דוגמה'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// הגדר הרשאת מפתח API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// הסר את ההערה שלהלן כדי להגדיר קידומת (לדוגמה Bearer) למפתח ה-API, אם נדרש
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם אישית, העבר את הלקוח שלך שמממש `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'comment_id' => 'comment_id_example', // string
    'external_id' => 'external_id_example', // string
    'event_type' => 'event_type_example', // string
    'type' => 'type_example', // string
    'domain' => 'domain_example', // string
    'attempt_count_gt' => 3.4, // float
];


try {
    $result = $apiInstance->getPendingWebhookEventCount($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getPendingWebhookEventCount: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]