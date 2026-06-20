## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| namespace | string | path | Ja |  |
| component | string | path | Ja |  |
| locale | string | query | Nej |  |
| useFullTranslationIds | boolean | query | Nej |  |

## Svar

Returnerer: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTranslationsResponse.php)

## Eksempel

[inline-code-attrs-start title = 'getTranslations Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, giv din klient som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
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