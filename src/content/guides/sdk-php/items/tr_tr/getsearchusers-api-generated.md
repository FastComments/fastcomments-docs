## Parametreler

| Ad | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| value | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationUserSearchResponse.php)

## Örnek

[inline-code-attrs-start title = 'getSearchUsers Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Özel bir http istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface` uygulayan istemcinizi geçin.
    // Bu isteğe bağlıdır, `GuzzleHttp\Client` varsayılan olarak kullanılacaktır.
    new GuzzleHttp\Client()
);
$value = 'value_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getSearchUsers($value, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getSearchUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]