## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| value | string | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationUserSearchResponse.php)

## Esempio

[inline-code-attrs-start title = 'Esempio di getSearchUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Se vuoi usare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale: verrà usato `GuzzleHttp\Client` come predefinito.
    new GuzzleHttp\Client()
);
$value = 'value_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getSearchUsers($value, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getSearchUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]