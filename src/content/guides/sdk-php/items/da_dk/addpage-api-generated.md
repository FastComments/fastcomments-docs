## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Respons

Returnerer: [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AddPageAPIResponse.php)

## Eksempel

[inline-code-attrs-start title = 'addPage Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurer API-nøglegodkendelse: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Fjern kommentaren nedenfor for at konfigurere præfiks (f.eks. Bearer) for API-nøglen, hvis nødvendigt
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, videregiv din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_api_page_data = new \FastComments\Client\Model\CreateAPIPageData(); // \FastComments\Client\Model\CreateAPIPageData

try {
    $result = $apiInstance->addPage($tenant_id, $create_api_page_data);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->addPage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---