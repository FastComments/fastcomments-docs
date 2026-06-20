## Parameters

| Name | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |

## Antwoord

Retourneert: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetV2PageReacts.php)

## Voorbeeld

[inline-code-attrs-start title = 'getV2PageReacts Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Als je een aangepaste HTTP-client wilt gebruiken, geef dan je client mee die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel; `GuzzleHttp\Client` wordt standaard gebruikt.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string

try {
    $result = $apiInstance->getV2PageReacts($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getV2PageReacts: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]