## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| largeInternalURLSanitized | string | query | Ja |  |

## Antwort

Gibt zurück: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GifGetLargeResponse.php)

## Beispiel

[inline-code-attrs-start title = 'getGifLarge Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird als Standard verwendet.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$large_internal_url_sanitized = 'large_internal_url_sanitized_example'; // string

try {
    $result = $apiInstance->getGifLarge($tenant_id, $large_internal_url_sanitized);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getGifLarge: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---