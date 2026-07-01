## Параметри

| Име | Тип | Локация | Задължително | Описание |
|------|------|----------|--------------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |
| id | string | query | Да |  |
| title | string | query | Не |  |

## Отговор

Връща: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateV1PageReact.php)

## Пример

[inline-code-attrs-start title = 'createV2PageReact Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако искате да използвате персонализиран HTTP клиент, предайте вашия клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е опционално, `GuzzleHttp\Client` ще се използва по подразбиране.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$id = 'id_example'; // string
$title = 'title_example'; // string


try {
    $result = $apiInstance->createV2PageReact($tenant_id, $url_id, $id, $title);
    print_r($result);
} catch (Exception $e) {
    echo 'Изключение при извикване на PublicApi->createV2PageReact: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---