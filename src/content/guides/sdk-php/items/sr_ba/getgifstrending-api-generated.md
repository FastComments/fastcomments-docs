## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| locale | string | query | Ne |  |
| rating | string | query | Ne |  |
| page | number | query | Ne |  |

## Odgovor

Vraća: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetGifsTrendingResponse.php)

## Primjer

[inline-code-attrs-start title = 'getGifsTrending Primjer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite koristiti prilagođeni http klijent, proslijedite vaš klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opcionalno, `GuzzleHttp\Client` će se koristiti kao zadani.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$locale = 'locale_example'; // string
$rating = 'rating_example'; // string
$page = 3.4; // float

try {
    $result = $apiInstance->getGifsTrending($tenant_id, $locale, $rating, $page);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getGifsTrending: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---