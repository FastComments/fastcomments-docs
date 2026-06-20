## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| namespace | string | path | Oui |  |
| component | string | path | Oui |  |
| locale | string | query | Non |  |
| useFullTranslationIds | boolean | query | Non |  |

## Réponse

Retourne : [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTranslationsResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple getTranslations'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, fournissez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);
$namespace = 'namespace_example'; // string
$component = 'component_example'; // string
$locale = 'locale_example'; // string
$use_full_translation_ids = True; // bool

try {
    $result = $apiInstance->getTranslations($namespace, $component, $locale, $use_full_translation_ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getTranslations: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]