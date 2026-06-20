## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| value | string | query | Non |  |
| filters | string | query | Non |  |
| searchFilters | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Retourne: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationCommentSearchResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple getSearchCommentsSummary'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);
$value = 'value_example'; // chaîne
$filters = 'filters_example'; // chaîne
$search_filters = 'search_filters_example'; // chaîne
$sso = 'sso_example'; // chaîne

try {
    $result = $apiInstance->getSearchCommentsSummary($value, $filters, $search_filters, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getSearchCommentsSummary: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]