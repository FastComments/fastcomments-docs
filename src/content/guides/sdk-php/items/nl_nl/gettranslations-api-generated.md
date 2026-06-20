## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| namespace | string | path | Ja |  |
| component | string | path | Ja |  |
| locale | string | query | Nee |  |
| useFullTranslationIds | boolean | query | Nee |  |

## Respons

Retourneert: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTranslationsResponse.php)

## Voorbeeld

[inline-code-attrs-start title = 'getTranslations Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Als u een aangepaste HTTP-client wilt gebruiken, geef dan uw client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel; `GuzzleHttp\Client` wordt standaard gebruikt.
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