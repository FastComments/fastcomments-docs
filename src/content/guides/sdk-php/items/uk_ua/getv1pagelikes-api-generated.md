## Parameters

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так |  |

## Response

Повертає: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetV1PageLikes.php)

## Приклад

[inline-code-attrs-start title = 'getV1PageLikes Приклад'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Якщо ви хочете використати кастомний HTTP-клієнт, передайте клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // рядок
$url_id = 'url_id_example'; // рядок

try {
    $result = $apiInstance->getV1PageLikes($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getV1PageLikes: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---