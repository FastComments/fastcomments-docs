## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|-------|-----|----------|----------|------|
| tenantId | string | query | Yes |  |
| urlId | string | query | Yes |  |

## Odgovor

Vraća: [`GetPageByURLIdAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPageByURLIdAPIResponse.php)

## Primjer

[inline-code-attrs-start title = 'Primjer getPageByURLId'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Konfigurišite autorizaciju API ključa: api_key
// Otkomentarišite donji red kako biste postavili prefiks (npr. Bearer) za API ključ, ako je potrebno
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ako želite koristiti prilagođeni http klijent, proslijedite vaš klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opciono, `GuzzleHttp\Client` će se koristiti kao podrazumevani.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string


try {
    $result = $apiInstance->getPageByURLId($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getPageByURLId: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]