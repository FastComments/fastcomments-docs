## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| search | string | query | Oui |  |
| locale | string | query | Non |  |
| rating | string | query | Non |  |
| page | number | query | Non |  |

## Réponse

Renvoie : [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetGifsSearchResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple de getGifsSearch'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si vous voulez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // chaîne
$search = 'search_example'; // chaîne
$locale = 'locale_example'; // chaîne
$rating = 'rating_example'; // chaîne
$page = 3.4; // nombre à virgule flottante

try {
    $result = $apiInstance->getGifsSearch($tenant_id, $search, $locale, $rating, $page);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getGifsSearch: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]