## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| value | string | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationSiteSearchResponse.php)

## Пример

[inline-code-attrs-start title = 'getSearchSites Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Ако искате да използвате персонализиран http клиент, подайте вашия клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е по избор, по подразбиране ще бъде използван `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$value = 'value_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getSearchSites($value, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getSearchSites: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]