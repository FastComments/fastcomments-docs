## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|--------------|-------------|----------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| id | string | query | Yes |  |

## Ответ

Возвращает: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetV2PageReactUsersResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример getV2PageReactUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Если вы хотите использовать пользовательский HTTP‑клиент, передайте ваш клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это необязательно, `GuzzleHttp\Client` будет использоваться по умолчанию.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // строка
$url_id = 'url_id_example'; // строка
$id = 'id_example'; // строка


try {
    $result = $apiInstance->getV2PageReactUsers($tenant_id, $url_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getV2PageReactUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]