Пожалуйста, выполните [процедуру установки](#installation--usage), а затем запустите следующее:

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



// Configure API key authorization: api_key
// Настройка авторизации с использованием API-ключа: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Раскомментируйте ниже, чтобы задать префикс (например Bearer) для API-ключа, если это необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Если вы хотите использовать свой HTTP-клиент, передайте клиент, который реализует `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$add_domain_config_params = new \FastComments\Client\Model\AddDomainConfigParams(); // \FastComments\Client\Model\AddDomainConfigParams

try {
    $result = $apiInstance->addDomainConfig($tenant_id, $add_domain_config_params);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->addDomainConfig: ', $e->getMessage(), PHP_EOL;
}

```