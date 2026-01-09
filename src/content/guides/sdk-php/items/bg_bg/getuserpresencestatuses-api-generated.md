## Параметри

| Име | Тип | Location | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| urlIdWS | string | query | Да |  |
| userIds | string | query | Да |  |

## Отговор

Връща: [`GetUserPresenceStatuses200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserPresenceStatuses200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример за getUserPresenceStatuses'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако искате да използвате персонален http клиент, подайте вашия клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е по избор, `GuzzleHttp\Client` ще бъде използван по подразбиране.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id_ws = 'url_id_ws_example'; // string
$user_ids = 'user_ids_example'; // string

try {
    $result = $apiInstance->getUserPresenceStatuses($tenant_id, $url_id_ws, $user_ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUserPresenceStatuses: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]