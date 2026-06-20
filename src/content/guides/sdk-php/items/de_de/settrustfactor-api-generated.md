## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| userId | string | query | Nein |  |
| trustFactor | string | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SetUserTrustFactorResponse.php)

## Beispiel

[inline-code-attrs-start title = 'setTrustFactor Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional; `GuzzleHttp\Client` wird standardmäßig verwendet.
    new GuzzleHttp\Client()
);
$user_id = 'user_id_example'; // string
$trust_factor = 'trust_factor_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->setTrustFactor($user_id, $trust_factor, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->setTrustFactor: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]