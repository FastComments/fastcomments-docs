## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| urlId | string | query | Evet |  |
| id | string | query | Evet |  |

## Yanıt

Döndürür: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateV1PageReact.php)

## Örnek

[inline-code-attrs-start title = 'deleteV2PageReact Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Özel bir HTTP istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface`'i uygulayan istemcinizi geçin.
    // Bu isteğe bağlıdır; varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$id = 'id_example'; // string

try {
    $result = $apiInstance->deleteV2PageReact($tenant_id, $url_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->deleteV2PageReact: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---