## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| userId | string | query | Не |  |

## Одговор

Враћа: [`GetTicket200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTicket200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример getTicket'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Конфигуришите ауторизацију API кључа: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Ако је потребно, откоментаришите ред испод да подесите префикс (нпр. Bearer) за API кључ
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако желите да користите прилагођени http клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционо, као подразумевани ће бити коришћен `GuzzleHttp\Client`.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$user_id = 'user_id_example'; // string

try {
    $result = $apiInstance->getTicket($tenant_id, $id, $user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getTicket: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]