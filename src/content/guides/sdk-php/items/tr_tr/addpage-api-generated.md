## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |

## Yanıt

Döndürür: [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AddPageAPIResponse.php)

## Örnek

[inline-code-attrs-start title = 'addPage Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API anahtarı yetkilendirmesini yapılandır: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Gerekirse API anahtarı için önek (örn. Bearer) ayarlamak için aşağıdaki satırı yorum dışı bırakın
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Özel bir HTTP istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface`'i uygulayan istemcinizi geçirin.
    // Bu isteğe bağlıdır, `GuzzleHttp\Client` varsayılan olarak kullanılacaktır.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$create_api_page_data = new \FastComments\Client\Model\CreateAPIPageData(); // \FastComments\Client\Model\CreateAPIPageData


try {
    $result = $apiInstance->addPage($tenant_id, $create_api_page_data);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->addPage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---