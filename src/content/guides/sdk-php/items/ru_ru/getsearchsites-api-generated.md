## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | query | Да |  |
| value | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationSiteSearchResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример getSearchSites'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



// $apiInstance = new FastComments\Client\Api\ModerationApi(
//     // Если вы хотите использовать собственный HTTP‑клиент, передайте ваш клиент, реализующий `GuzzleHttp\ClientInterface`.
//     // Это опционально, по умолчанию будет использован `GuzzleHttp\Client`.
//     new GuzzleHttp\Client()
// );

$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Если вы хотите использовать собственный HTTP‑клиент, передайте ваш клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это опционально, `GuzzleHttp\Client` будет использоваться по умолчанию.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'value' => 'value_example', // string
    'sso' => 'sso_example', // string
];


try {
    $result = $apiInstance->getSearchSites($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getSearchSites: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]