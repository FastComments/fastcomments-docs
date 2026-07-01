## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|-------------|--------------|-------------|
| tenantId | string | query | Oui |  |
| text-search | string | query | Non |  |
| byIPFromComment | string | query | Non |  |
| filter | string | query | Non |  |
| searchFilters | string | query | Non |  |
| demo | boolean | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPICountCommentsResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple getCount'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Si vous voulez utiliser un client HTTP personnalisé, transmettez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // chaîne
$options = [
    'text_search' => 'text_search_example', // chaîne
    'by_ip_from_comment' => 'by_ip_from_comment_example', // chaîne
    'filter' => 'filter_example', // chaîne
    'search_filters' => 'search_filters_example', // chaîne
    'demo' => True, // booléen
    'sso' => 'sso_example', // chaîne
];


try {
    $result = $apiInstance->getCount($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]