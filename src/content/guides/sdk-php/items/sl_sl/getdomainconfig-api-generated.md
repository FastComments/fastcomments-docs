## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| domain | string | path | Da |  |

## Odgovor

Vrne: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetDomainConfigResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer getDomainConfig'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurirajte pooblastilo API ključa: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Po potrebi odkomentirajte spodaj za nastavitev predpone (npr. Bearer) za API ključ
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Če želite uporabiti lasten HTTP odjemalec, posredujte svoj odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
    // To je neobvezno; privzeto bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$domain = 'domain_example'; // string

try {
    $result = $apiInstance->getDomainConfig($tenant_id, $domain);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getDomainConfig: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---