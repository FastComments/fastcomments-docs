## Параметри

| Назва | Тип | Розташування | Обовʼязковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Відповідь

Повертає: [`GetCachedNotificationCountResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCachedNotificationCountResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад getCachedNotificationCount'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Налаштуйте авторизацію за допомогою API‑ключа: api_key
// Розкоментуйте нижче, щоб встановити префікс (наприклад, Bearer) для API‑ключа, якщо потрібно
// Якщо ви хочете використати власний HTTP‑клієнт, передайте ваш клієнт, який реалізує `GuzzleHttp\ClientInterface`.
// Це необов’язково, `GuzzleHttp\Client` буде використано за замовчуванням.

$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Якщо ви хочете використати власний HTTP‑клієнт, передайте ваш клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов’язково, `GuzzleHttp\Client` буде використано за замовчуванням.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string

try {
    $result = $apiInstance->getCachedNotificationCount($tenant_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getCachedNotificationCount: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]