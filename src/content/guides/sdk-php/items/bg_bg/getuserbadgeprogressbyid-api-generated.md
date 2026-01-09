## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Отговор

Връща: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserBadgeProgressById200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример getUserBadgeProgressById'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Конфигуриране на удостоверяване с API ключ: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Разкоментирайте долния ред, за да зададете префикс (напр. Bearer) за API ключа, ако е необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако искате да използвате персонализиран HTTP клиент, предайте вашия клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е по избор; `GuzzleHttp\Client` ще се използва по подразбиране.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string

try {
    $result = $apiInstance->getUserBadgeProgressById($tenant_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getUserBadgeProgressById: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]