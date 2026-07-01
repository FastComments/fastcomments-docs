## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Ответ

Returns: [`APIGetCommentResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIGetCommentResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример getComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Настройте авторизацию API-ключа: api_key
// Раскомментируйте ниже, чтобы установить префикс (например, Bearer) для API-ключа, если необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Если вы хотите использовать пользовательский HTTP‑клиент, передайте ваш клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это необязательно, `GuzzleHttp\Client` будет использоваться по умолчанию.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string


try {
    $result = $apiInstance->getComment($tenant_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]