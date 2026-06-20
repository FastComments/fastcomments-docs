## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserNotificationCountResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример за getUserNotificationCount'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако искате да използвате собствен HTTP клиент, подайте клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е по избор, като по подразбиране ще се използва `GuzzleHttp\Client`.
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