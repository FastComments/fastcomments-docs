## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Отговор

Връща: [`CreateEmailTemplateResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateEmailTemplateResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример за createEmailTemplate'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Конфигуриране на упълномощаване с API ключ: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Премахнете коментара по-долу, за да зададете префикс (напр. Bearer) за API ключа, ако е необходимо
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако искате да използвате персонализиран HTTP клиент, подайте клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е по избор, `GuzzleHttp\Client` ще бъде използван по подразбиране.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$create_email_template_body = new \FastComments\Client\Model\CreateEmailTemplateBody(); // \FastComments\Client\Model\CreateEmailTemplateBody

try {
    $result = $apiInstance->createEmailTemplate($tenant_id, $create_email_template_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createEmailTemplate: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]