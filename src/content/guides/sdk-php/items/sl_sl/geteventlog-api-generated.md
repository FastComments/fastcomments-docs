req
tenantId
urlId
userIdWS

## Parametri

| Name | Tip | Location | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da |  |
| userIdWS | string | query | Da |  |
| startTime | integer | query | Da |  |
| endTime | integer | query | Da |  |

## Odgovor

Vrne: [`GetEventLog200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetEventLog200Response.php)

## Primer

[inline-code-attrs-start title = 'getEventLog Primer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Če želite uporabljati prilagojen HTTP odjemalec, posredujte svoj odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
    // To je neobvezno, kot privzeti se bo uporabil `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$user_id_ws = 'user_id_ws_example'; // string
$start_time = 56; // int
$end_time = 56; // int

try {
    $result = $apiInstance->getEventLog($tenant_id, $url_id, $user_id_ws, $start_time, $end_time);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getEventLog: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]