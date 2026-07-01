## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da |  |

## Odziv

Returns: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetV1PageLikes.php)

## Primer

[inline-code-attrs-start title = 'Primer getV1PageLikes'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Če želite uporabiti prilagojeni HTTP odjemalec, podajte svoj odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
    // To je neobvezno, po privzetku bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string


try {
    $result = $apiInstance->getV1PageLikes($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getV1PageLikes: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]