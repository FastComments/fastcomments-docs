## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| limit | number | query | No |  |
| skip | number | query | No |  |
| order | string | query | No |  |
| after | number | query | No |  |
| before | number | query | No |  |

## Odgovor

Vraća: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetAuditLogsResponse.php)

## Primjer

[inline-code-attrs-start title = 'Primjer getAuditLogs'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurišite autorizaciju API ključa: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Otkomentarišite ispod da podesite prefiks (npr. Bearer) za API ključ, ako je potrebno
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ako želite koristiti prilagođeni HTTP klijent, proslijedite vaš klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opciono, po defaultu će biti korišćen `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$limit = 3.4; // float
$skip = 3.4; // float
$order = new \FastComments\Client\Model\\FastComments\Client\Model\SORTDIR(); // \FastComments\Client\Model\SORTDIR
$after = 3.4; // float
$before = 3.4; // float

try {
    $result = $apiInstance->getAuditLogs($tenant_id, $limit, $skip, $order, $after, $before);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getAuditLogs: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]