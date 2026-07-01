## Parametreler

| Ad | Tür | Konum | Gereklidir | Açıklama |
|------|------|----------|------------|-------------|
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
    // Özel bir HTTP istemcisi kullanmak isterseniz, `GuzzleHttp\ClientInterface`'i uygulayan istemcinizi geçirin.
    // Bu isteğe bağlıdır, varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
    new GuzzleHttp\Client()
);

$namespace = 'namespace_example'; // string
$component = 'component_example'; // string
$options = [
    'locale' => 'locale_example', // string
    'use_full_translation_ids' => True, // bool
];


try {
    $result = $apiInstance->getTranslations($namespace, $component, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getTranslations: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]