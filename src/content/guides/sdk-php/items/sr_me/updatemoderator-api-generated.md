## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Одговор

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример updateModerator'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Ако је потребно, откоментаришите испод да бисте подесили префикс (нпр. Bearer) за API кључ
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако желите да користите прилагођени HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, подразумевано ће бити коришћен `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$update_moderator_body = new \FastComments\Client\Model\UpdateModeratorBody(); // \FastComments\Client\Model\UpdateModeratorBody

try {
    $result = $apiInstance->updateModerator($tenant_id, $id, $update_moderator_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->updateModerator: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]