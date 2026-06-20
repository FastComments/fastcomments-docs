## Parameters

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| urlId | string | query | Oui |  |
| sso | string | query | Non |  |

## Réponse

Retourne: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple de putCloseThread'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);
$url_id = 'url_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->putCloseThread($url_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->putCloseThread: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]