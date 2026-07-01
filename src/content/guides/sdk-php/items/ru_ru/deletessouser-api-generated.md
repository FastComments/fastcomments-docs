## Параметры

| Имя | Тип | Location | Required | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| deleteComments | boolean | query | Нет |  |
| commentDeleteMode | string | query | Нет |  |

## Ответ

Возвращает: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteSSOUserAPIResponse.php)

## Пример

[inline-code-attrs-start title = 'deleteSSOUser Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Настройте авторизацию ключа API: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Раскомментируйте ниже, чтобы установить префикс (например, Bearer) для ключа API, если необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Если вы хотите использовать пользовательского HTTP-клиента, передайте ваш клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это опционально, `GuzzleHttp\Client` будет использоваться по умолчанию.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // строка
$id = 'id_example'; // строка
$options = [
    'delete_comments' => True, // логическое
    'comment_delete_mode' => 'comment_delete_mode_example', // строка
];


try {
    $result = $apiInstance->deleteSSOUser($tenant_id, $id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteSSOUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]