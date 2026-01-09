## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`GetUserNotificationCount200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserNotificationCount200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример getUserNotificationCount'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Если вы хотите использовать пользовательский HTTP-клиент, передайте ваш клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getUserNotificationCount($tenant_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUserNotificationCount: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---