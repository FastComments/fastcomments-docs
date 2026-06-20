## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |

## Yanıt

Döndürür: [`CreateQuestionResultResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateQuestionResultResponse.php)

## Örnek

[inline-code-attrs-start title = 'createQuestionResult Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API anahtarı yetkilendirmesini yapılandırın: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Gerekirse API anahtarı için önek (örn. Bearer) ayarlamak üzere aşağıdaki satırın yorumunu kaldırın
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Özel bir HTTP istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface`'i uygulayan istemcinizi geçin.
    // Bu isteğe bağlıdır; varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_question_result_body = new \FastComments\Client\Model\CreateQuestionResultBody(); // \FastComments\Client\Model\CreateQuestionResultBody

try {
    $result = $apiInstance->createQuestionResult($tenant_id, $create_question_result_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createQuestionResult: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]