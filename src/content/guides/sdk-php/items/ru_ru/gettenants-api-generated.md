## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| meta | string | query | Нет |  |
| skip | number | query | Нет |  |

## Ответ

Возвращает: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenantsResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример getTenants'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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

$tenant_id = 'tenant_id_example'; // строка
$options = [
    'meta' => 'meta_example', // строка
    'skip' => 3.4, // число с плавающей точкой
];


try {
    $result = $apiInstance->getTenants($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTenants: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]