## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| search | string | query | Да |  |
| locale | string | query | Не |  |
| rating | string | query | Не |  |
| page | number | query | Не |  |

## Одговор

Враћа: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetGifsSearchResponse.php)

## Примјер

[inline-code-attrs-start title = 'getGifsSearch Примјер'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако желите да користите прилагођеног HTTP клијента, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, као подразумевани биће коришћен `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // низ
$search = 'search_example'; // низ
$locale = 'locale_example'; // низ
$rating = 'rating_example'; // низ
$page = 3.4; // број (float)

try {
    $result = $apiInstance->getGifsSearch($tenant_id, $search, $locale, $rating, $page);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getGifsSearch: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]