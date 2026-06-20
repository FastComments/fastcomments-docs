## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | path | Да |  |

## Отговор

Връща: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIGetUserBadgeProgressResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример за getUserBadgeProgressByUserId'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Конфигуриране на удостоверяването с API ключ: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Разкоментирайте по-долу, за да зададете префикс (напр. Bearer) за API ключа, ако е необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако искате да използвате собствен HTTP клиент, подайте ваш клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е по избор, `GuzzleHttp\Client` ще се използва като подразбиране.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string

try {
    $result = $apiInstance->getUserBadgeProgressByUserId($tenant_id, $user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getUserBadgeProgressByUserId: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]