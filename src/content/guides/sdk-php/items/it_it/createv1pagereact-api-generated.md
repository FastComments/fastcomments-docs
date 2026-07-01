## Parametri

| Nome | Tipo | Posizione | Richiesto | Descrizione |
|------|------|-----------|-----------|-------------|
| tenantId | string | path | Sì |  |
| urlId | string | query | Sì |  |
| title | string | query | No |  |

## Risposta

Restituisce: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateV1PageReact.php)

## Esempio

[inline-code-attrs-start title = 'createV1PageReact Esempio'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Se vuoi usare un client http personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, `GuzzleHttp\Client` verrà usato di default.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$title = 'title_example'; // string


try {
    $result = $apiInstance->createV1PageReact($tenant_id, $url_id, $title);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->createV1PageReact: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]