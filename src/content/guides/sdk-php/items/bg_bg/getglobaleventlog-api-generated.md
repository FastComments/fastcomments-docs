req
tenantId
urlId
userIdWS

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |
| userIdWS | string | query | Да |  |
| startTime | integer | query | Да |  |
| endTime | integer | query | Не |  |

## Отговор

Връща: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetEventLogResponse.php)

## Пример

[inline-code-attrs-start title = 'getGlobalEventLog Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако искате да използвате персонализиран HTTP клиент, подайте вашия клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е по избор, по подразбиране ще се използва `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$user_id_ws = 'user_id_ws_example'; // string
$start_time = 56; // int
$end_time = 56; // int

try {
    $result = $apiInstance->getGlobalEventLog($tenant_id, $url_id, $user_id_ws, $start_time, $end_time);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getGlobalEventLog: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]