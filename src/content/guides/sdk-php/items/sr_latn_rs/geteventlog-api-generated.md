req
tenantId
urlId
userIdWS

## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| userIdWS | string | query | Yes |  |
| startTime | integer | query | Yes |  |
| endTime | integer | query | No |  |

## Odgovor

Vraća: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetEventLogResponse.php)

## Primer

[inline-code-attrs-start title = 'getEventLog Primer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite da koristite prilagođeni http klijent, prosledite vaš klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opciono, `GuzzleHttp\Client` će biti korišćen po defaultu.
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