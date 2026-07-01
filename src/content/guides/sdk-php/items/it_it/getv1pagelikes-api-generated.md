## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | path | Sì |  |
| urlId | string | query | Sì |  |

## Risposta

Restituisce: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetV1PageLikes.php)

## Esempio

[inline-code-attrs-start title = 'Esempio getV1PageLikes'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se desideri usare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // È opzionale, verrà usato `GuzzleHttp\Client` come predefinito.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // stringa
$url_id = 'url_id_example'; // stringa


try {
    $result = $apiInstance->getV1PageLikes($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getV1PageLikes: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]