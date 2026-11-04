Масова інформація про користувачів для орендаря. За заданими userIds повертає інформацію для відображення з User / SSOUser.
Використовується віджетом коментарів для збагачення користувачів, які щойно з'явилися через подію присутності.
Без контексту сторінки: приватність застосовується однаково (приватні профілі маскуються).

## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| ids | string | query | Так | userIds, розділені комами. |

## Відповідь

Повертає: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersInfoResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад getUsersInfo'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Якщо хочете використати власний http-клієнт, передайте свій клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$ids = 'ids_example'; // string | userIds, розділені комами.


try {
    $result = $apiInstance->getUsersInfo($tenant_id, $ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUsersInfo: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]