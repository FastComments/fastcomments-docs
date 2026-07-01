## Response

Returns: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример logoutPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Если вы хотите использовать собственный HTTP‑клиент, передайте ваш клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это опционально, по умолчанию будет использоваться `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);



try {
    $result = $apiInstance->logoutPublic();
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->logoutPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]