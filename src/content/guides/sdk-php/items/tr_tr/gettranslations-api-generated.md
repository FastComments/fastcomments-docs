## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| namespace | string | path | Evet |  |
| component | string | path | Evet |  |
| locale | string | query | Hayır |  |
| useFullTranslationIds | boolean | query | Hayır |  |

## Yanıt

Döndürür: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTranslationsResponse.php)

## Örnek

[inline-code-attrs-start title = 'getTranslations Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
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

---