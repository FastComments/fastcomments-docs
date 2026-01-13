## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Ответ

Возвращает: [`CreateModerator200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateModerator200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример createModerator'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Настроить авторизацию по API-ключу: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Раскомментируйте ниже, чтобы настроить префикс (например Bearer) для API-ключа, если нужно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Если вы хотите использовать пользовательский HTTP-клиент, передайте клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_moderator_body = new \FastComments\Client\Model\CreateModeratorBody(); // \FastComments\Client\Model\CreateModeratorBody

try {
    $result = $apiInstance->createModerator($tenant_id, $create_moderator_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createModerator: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]