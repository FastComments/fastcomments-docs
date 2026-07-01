## Parameter

| Name     | Typ    | Ort   | Erforderlich | Beschreibung |
|----------|--------|-------|--------------|--------------|
| tenantId | string | path  | Ja           |              |
| locale   | string | query | Nein         |              |
| rating   | string | query | Nein         |              |
| page     | number | query | Nein         |              |

## Antwort

Rückgabe: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetGifsTrendingResponse.php)

## Beispiel

[inline-code-attrs-start title = 'getGifsTrending Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // Zeichenkette
$options = [
    'locale' => 'locale_example', // Zeichenkette
    'rating' => 'rating_example', // Zeichenkette
    'page' => 3.4, // Fließkommazahl
];


try {
    $result = $apiInstance->getGifsTrending($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getGifsTrending: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]