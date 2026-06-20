req
tenantId
urlId
userIdWS

## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| userIdWS | string | query | Yes |  |
| startTime | integer | query | Yes |  |
| endTime | integer | query | No |  |

## תגובה

מחזיר: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetEventLogResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getGlobalEventLog'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם אישית, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי; `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // מחרוזת
$url_id = 'url_id_example'; // מחרוזת
$user_id_ws = 'user_id_ws_example'; // מחרוזת
$start_time = 56; // מספר שלם
$end_time = 56; // מספר שלם

try {
    $result = $apiInstance->getGlobalEventLog($tenant_id, $url_id, $user_id_ws, $start_time, $end_time);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getGlobalEventLog: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]