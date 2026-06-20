---
## Параметры

| Имя | Тип | Местоположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| locale | string | query | Нет |  |
| rating | string | query | Нет |  |
| page | number | query | Нет |  |

## Ответ

Возвращает: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetGifsTrendingResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример getGifsTrending'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Если вы хотите использовать пользовательский HTTP-клиент, передайте клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это необязательно — по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // строка
$locale = 'locale_example'; // строка
$rating = 'rating_example'; // строка
$page = 3.4; // число с плавающей точкой

try {
    $result = $apiInstance->getGifsTrending($tenant_id, $locale, $rating, $page);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getGifsTrending: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---