## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Не |  |
| state | number | query | Не |  |
| skip | number | query | Не |  |
| limit | number | query | Не |  |

## Одговор

Враћа: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTickets200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример getTickets'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Уклоните коментар испод да подесите префикс (нпр. Bearer) за API кључ, ако је потребно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако желите да користите прилагођени HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, `GuzzleHttp\Client` ће бити коришћен као подразумевани.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // стринг
$user_id = 'user_id_example'; // стринг
$state = 3.4; // флоат
$skip = 3.4; // флоат
$limit = 3.4; // флоат

try {
    $result = $apiInstance->getTickets($tenant_id, $user_id, $state, $skip, $limit);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTickets: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---