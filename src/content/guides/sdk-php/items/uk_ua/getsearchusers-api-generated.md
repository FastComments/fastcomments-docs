## Parameters

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|--------------|--------------|------|
| tenantId | string | query | Так |  |
| value | string | query | Ні |  |
| sso | string | query | Ні |  |

## Response

Повертає: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationUserSearchResponse.php)

## Example

[inline-code-attrs-start title = 'Приклад getSearchUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Якщо ви хочете використовувати власний http‑клієнт, передайте ваш клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов’язково, `GuzzleHttp\Client` буде використаний за замовчуванням.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // рядок
$options = [
    'value' => 'value_example', // рядок
    'sso' => 'sso_example', // рядок
];


try {
    $result = $apiInstance->getSearchUsers($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getSearchUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]