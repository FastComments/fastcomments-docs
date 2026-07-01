## Параметри

| Име | Тип | Местоположение | Задължителен | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| skip | number | query | No |  |

## Отговор

Връща: [`GetEmailTemplateRenderErrorsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetEmailTemplateRenderErrorsResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример за getEmailTemplateRenderErrors'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Конфигуриране на API ключ за оторизация: api_key
// Разкоментирайте по-долу, за да зададете префикс (например Bearer) за API ключа, ако е необходимо
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако искате да използвате персонализиран HTTP клиент, подайте вашия клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е опционално, `GuzzleHttp\Client` ще се използва по подразбиране.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // низ
$id = 'id_example'; // низ
$skip = 3.4; // число с плаваща запетая


try {
    $result = $apiInstance->getEmailTemplateRenderErrors($tenant_id, $id, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getEmailTemplateRenderErrors: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]