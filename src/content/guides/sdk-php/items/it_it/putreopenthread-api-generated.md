## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| urlId | string | query | Sì |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Esempio

[inline-code-attrs-start title = 'Esempio putReopenThread'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Se desideri usare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, verrà usato `GuzzleHttp\Client` come impostazione predefinita.
    new GuzzleHttp\Client()
);
$url_id = 'url_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->putReopenThread($url_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->putReopenThread: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]