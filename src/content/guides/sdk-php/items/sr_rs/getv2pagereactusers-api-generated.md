## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da |  |
| id | string | query | Da |  |

## Odgovor

Vraća: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetV2PageReactUsersResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer getV2PageReactUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite koristiti prilagođeni http klijent, prosledite vaš klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opciono, `GuzzleHttp\Client` će se koristiti kao podrazumevani.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$id = 'id_example'; // string


try {
    $result = $apiInstance->getV2PageReactUsers($tenant_id, $url_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getV2PageReactUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---