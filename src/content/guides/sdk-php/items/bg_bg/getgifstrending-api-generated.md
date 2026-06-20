## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| locale | string | query | No |  |
| rating | string | query | No |  |
| page | number | query | No |  |

## Отговор

Връща: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetGifsTrendingResponse.php)

## Пример

[inline-code-attrs-start title = 'getGifsTrending Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако искате да използвате собствен HTTP клиент, подайте клиента си, който реализира `GuzzleHttp\ClientInterface`.
    // Това е незадължително, по подразбиране ще се използва `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$locale = 'locale_example'; // string
$rating = 'rating_example'; // string
$page = 3.4; // float

try {
    $result = $apiInstance->getGifsTrending($tenant_id, $locale, $rating, $page);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getGifsTrending: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]