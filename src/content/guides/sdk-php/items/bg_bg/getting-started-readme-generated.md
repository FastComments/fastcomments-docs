---
Моля, следвайте [процедурата за инсталиране](#installation-usage-readme-generated) и след това изпълнете следното:

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



// Конфигуриране на авторизация с API ключ: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Разкоментирайте по-долу, за да зададете префикс (например Bearer) за API ключа, ако е необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако искате да използвате собствен HTTP клиент, подайте ваш клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е по избор, по подразбиране ще се използва `GuzzleHttp\Client`.
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
---