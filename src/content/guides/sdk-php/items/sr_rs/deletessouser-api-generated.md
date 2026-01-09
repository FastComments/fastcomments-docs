## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| deleteComments | boolean | query | Не |  |
| commentDeleteMode | string | query | Не |  |

## Одговор

Враћа: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteSSOUserAPIResponse.php)

## Пример

[inline-code-attrs-start title = 'deleteSSOUser пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Конфигуришите ауторизацију API кључа: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Откоментујте доле да бисте подесили префикс (нпр. Bearer) за API кључ, ако је потребно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако желите да користите прилагођени HTTP клијент, проследите свој клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционо, `GuzzleHttp\Client` ће бити коришћен као подразумевани.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$delete_comments = True; // bool
$comment_delete_mode = 'comment_delete_mode_example'; // string

try {
    $result = $apiInstance->deleteSSOUser($tenant_id, $id, $delete_comments, $comment_delete_mode);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteSSOUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]