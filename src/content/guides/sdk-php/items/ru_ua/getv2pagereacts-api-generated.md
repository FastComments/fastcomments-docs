## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | путь | Да |  |
| urlId | string | параметр запроса | Да |  |

## Ответ

Возвращает: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetV2PageReacts.php)

## Пример

[inline-code-attrs-start title = 'Пример getV2PageReacts'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Если вы хотите использовать пользовательский HTTP-клиент, передайте ваш клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // строка
$url_id = 'url_id_example'; // строка

try {
    $result = $apiInstance->getV2PageReacts($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getV2PageReacts: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---