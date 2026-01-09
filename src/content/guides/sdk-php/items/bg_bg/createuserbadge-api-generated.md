## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Отговор

Връща: [`CreateUserBadge200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateUserBadge200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример за createUserBadge'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Ако е необходимо, премахнете коментара по-долу, за да зададете префикс (напр. Bearer) за API ключа
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако искате да използвате потребителски HTTP клиент, предайте вашия клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е по избор, по подразбиране ще се използва `GuzzleHttp\Client`.
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