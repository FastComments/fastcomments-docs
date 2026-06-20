## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |

## Отговор

Връща: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetV1PageLikes.php)

## Пример

[inline-code-attrs-start title = 'Пример за getV1PageLikes'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако искате да използвате персонализиран HTTP клиент, предайте своя клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е по избор, като по подразбиране ще се използва `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string

try {
    $result = $apiInstance->getV1PageLikes($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getV1PageLikes: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---