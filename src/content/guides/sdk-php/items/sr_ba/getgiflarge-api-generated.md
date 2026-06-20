## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| largeInternalURLSanitized | string | query | Да |  |

## Одговор

Враћа: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GifGetLargeResponse.php)

## Пример

[inline-code-attrs-start title = 'getGifLarge Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако желите користити прилагођени HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, као подразумевани биће коришћен `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$large_internal_url_sanitized = 'large_internal_url_sanitized_example'; // string

try {
    $result = $apiInstance->getGifLarge($tenant_id, $large_internal_url_sanitized);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getGifLarge: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---