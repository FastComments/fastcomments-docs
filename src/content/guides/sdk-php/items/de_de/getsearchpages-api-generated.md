## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| value | string | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationPageSearchResponse.php)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getSearchPages'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Wenn Sie einen benutzerdefinierten http-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
    new GuzzleHttp\Client()
);
$value = 'value_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getSearchPages($value, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getSearchPages: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]