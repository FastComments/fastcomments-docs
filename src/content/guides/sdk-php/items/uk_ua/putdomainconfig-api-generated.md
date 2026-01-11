## Параметри

| Назва | Тип | Розташування | Обов'язкове | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| domainToUpdate | string | path | Так |  |

## Відповідь

Повертає: [`GetDomainConfig200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetDomainConfig200Response.php)

## Приклад

[inline-code-attrs-start title = 'Приклад putDomainConfig'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Налаштування авторизації API-ключа: api_key
// Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API-ключа, якщо потрібно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте ваш клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$domain_to_update = 'domain_to_update_example'; // string
$update_domain_config_params = new \FastComments\Client\Model\UpdateDomainConfigParams(); // \FastComments\Client\Model\UpdateDomainConfigParams

try {
    $result = $apiInstance->putDomainConfig($tenant_id, $domain_to_update, $update_domain_config_params);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->putDomainConfig: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]