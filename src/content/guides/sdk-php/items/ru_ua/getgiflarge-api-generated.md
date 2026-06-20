## Параметры

| Имя | Тип | Местоположение | Обязательное | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| largeInternalURLSanitized | string | query | Да |  |

## Ответ

Возвращает: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GifGetLargeResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример getGifLarge'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Если вы хотите использовать пользовательский HTTP-клиент, передайте ваш клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // строка
$large_internal_url_sanitized = 'large_internal_url_sanitized_example'; // строка

try {
    $result = $apiInstance->getGifLarge($tenant_id, $large_internal_url_sanitized);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getGifLarge: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]