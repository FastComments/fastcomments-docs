## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |
| userId | string | query | Hayır |  |

## Yanıt

Döndürür: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UpdateSubscriptionAPIResponse.php)

## Örnek

[inline-code-attrs-start title = 'updateSubscription Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// API anahtar yetkilendirmesini yapılandırın: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Gerekirse API anahtarı için önek (ör. Bearer) ayarlamak üzere aşağıdaki satırın yorumunu kaldırın
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Özel bir http istemcusu kullanmak istiyorsanız, `GuzzleHttp\ClientInterface`'i uygulayan istemcinizi geçin.
    // Bu isteğe bağlıdır, varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$update_api_user_subscription_data = new \FastComments\Client\Model\UpdateAPIUserSubscriptionData(); // \FastComments\Client\Model\UpdateAPIUserSubscriptionData
$user_id = 'user_id_example'; // string

try {
    $result = $apiInstance->updateSubscription($tenant_id, $id, $update_api_user_subscription_data, $user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->updateSubscription: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]