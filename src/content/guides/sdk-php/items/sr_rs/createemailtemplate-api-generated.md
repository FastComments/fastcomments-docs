## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string query Да  |

## Одговор

Враћа: [`CreateEmailTemplateResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateEmailTemplateResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример createEmailTemplate'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Конфигуриши ауторизацију API кључа: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Одкоментариши испод да подесиш префикс (нпр. Bearer) за API кључ, ако је потребно
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Ако желиш користити прилагођени http клијент, проследи свој клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционо, `GuzzleHttp\Client` ће се користити као подразумевано.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // стринг
$create_email_template_body = new \FastComments\Client\Model\CreateEmailTemplateBody(); // \FastComments\Client\Model\CreateEmailTemplateBody


try {
    $result = $apiInstance->createEmailTemplate($tenant_id, $create_email_template_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createEmailTemplate: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---