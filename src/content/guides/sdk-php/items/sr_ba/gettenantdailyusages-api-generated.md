## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| yearNumber | number | query | Не |  |
| monthNumber | number | query | Не |  |
| dayNumber | number | query | Не |  |
| skip | number | query | Не |  |

## Одговор

Враћа: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenantDailyUsagesResponse.php)

## Пример

[inline-code-attrs-start title = 'getTenantDailyUsages Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако желите користити прилагођен http клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционо, `GuzzleHttp\Client` ће бити коришћен као подразумевани.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$year_number = 3.4; // float
$month_number = 3.4; // float
$day_number = 3.4; // float
$skip = 3.4; // float

try {
    $result = $apiInstance->getTenantDailyUsages($tenant_id, $year_number, $month_number, $day_number, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTenantDailyUsages: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]