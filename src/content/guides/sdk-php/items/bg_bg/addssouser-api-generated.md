## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Отговор

Връща: [`AddSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AddSSOUserAPIResponse.php)

## Пример

[inline-code-attrs-start title = 'addSSOUser Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// Конфигурирайте упълномощаване с API ключ: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Разкоментирайте по-долу, за да зададете префикс (например Bearer) за API ключа, ако е необходимо
// If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
// Ако искате да използвате персонализиран HTTP клиент, предайте вашия клиент, който имплементира `GuzzleHttp\ClientInterface`.
// This is optional, `GuzzleHttp\Client` will be used as default.
// Това е опционално, `GuzzleHttp\Client` ще бъде използван като по подразбиране.

$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // Това е опционално, `GuzzleHttp\Client` ще бъде използван като по подразбиране.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$create_apisso_user_data = new \FastComments\Client\Model\CreateAPISSOUserData(); // \FastComments\Client\Model\CreateAPISSOUserData


try {
    $result = $apiInstance->addSSOUser($tenant_id, $create_apisso_user_data);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->addSSOUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]