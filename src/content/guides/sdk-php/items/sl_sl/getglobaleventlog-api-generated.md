req
tenantId
urlId
userIdWS

## Parametri

| Ime | Tip | Lokacija | Zahtevano | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da |  |
| userIdWS | string | query | Da |  |
| startTime | integer | query | Da |  |
| endTime | integer | query | Ne |  |

## Odgovor

Vrne: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetEventLogResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer getGlobalEventLog'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



// Če želite uporabiti lastnega http klienta, posredujte klienta, ki implementira `GuzzleHttp\ClientInterface`.
// To je opcijsko, privzeto bo uporabljen `GuzzleHttp\Client`.
$apiInstance = new FastComments\Client\Api\PublicApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
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