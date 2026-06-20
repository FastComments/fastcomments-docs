## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Ответ

Возвращает: [`APIEmptySuccessResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptySuccessResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример updateUserBadge'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Настройка авторизации API-ключом: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Раскомментируйте строку ниже, чтобы установить префикс (например, Bearer) для API-ключа, если это необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Если вы хотите использовать пользовательский HTTP-клиент, передайте ваш клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использоваться `GuzzleHttp\Client`.
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