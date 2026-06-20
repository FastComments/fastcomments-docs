## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Отговор

Връща: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptySuccessResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример за updateUserBadge'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Конфигуриране на удостоверяване с API ключ: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Разкоментирайте по-долу, за да зададете префикс (например Bearer) за API ключа, ако е необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако искате да използвате персонализиран HTTP клиент, предайте ваш клиент, който реализира `GuzzleHttp\ClientInterface`.
    // Това е по избор, по подразбиране ще бъде използван `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$update_user_badge_params = new \FastComments\Client\Model\UpdateUserBadgeParams(); // \FastComments\Client\Model\UpdateUserBadgeParams

try {
    $result = $apiInstance->updateUserBadge($tenant_id, $id, $update_user_badge_params);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->updateUserBadge: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---