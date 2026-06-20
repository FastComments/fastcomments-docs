## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da |  |
| id | string | query | Da |  |
| title | string | query | Ne |  |

## Odgovor

Vraća: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateV1PageReact.php)

## Primer

[inline-code-attrs-start title = 'Primer createV2PageReact'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite da koristite prilagođeni HTTP klijent, prosledite klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opciono, kao podrazumevani će biti korišćen `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$id = 'id_example'; // string
$title = 'title_example'; // string

try {
    $result = $apiInstance->createV2PageReact($tenant_id, $url_id, $id, $title);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->createV2PageReact: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---