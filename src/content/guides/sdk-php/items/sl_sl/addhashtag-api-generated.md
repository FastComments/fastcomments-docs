## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ne |  |

## Odgovor

Vrača: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AddHashTag200Response.php)

## Primer

[inline-code-attrs-start title = 'Primer addHashTag'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurirajte pooblastilo API key: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za API key, če je potrebno
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Če želite uporabiti lastnega HTTP odjemalca, posredujte odjemalca, ki implementira `GuzzleHttp\ClientInterface`.
    // To ni obvezno, kot privzeti se bo uporabil `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_hash_tag_body = new \FastComments\Client\Model\CreateHashTagBody(); // \FastComments\Client\Model\CreateHashTagBody

try {
    $result = $apiInstance->addHashTag($tenant_id, $create_hash_tag_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->addHashTag: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]