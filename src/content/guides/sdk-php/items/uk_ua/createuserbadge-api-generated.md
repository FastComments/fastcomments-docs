## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |

## Відповідь

Повертає: [`APICreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APICreateUserBadgeResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад createUserBadge'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Налаштуйте авторизацію ключа API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для ключа API, якщо потрібно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Якщо ви хочете використати власний http-клієнт, передайте клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використаний `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_user_badge_params = new \FastComments\Client\Model\CreateUserBadgeParams(); // \FastComments\Client\Model\CreateUserBadgeParams

try {
    $result = $apiInstance->createUserBadge($tenant_id, $create_user_badge_params);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createUserBadge: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]