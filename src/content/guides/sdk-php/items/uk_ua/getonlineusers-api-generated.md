Користувачі, що наразі онлайн на сторінці: люди, чиї websocket session підписані на цю сторінку в даний момент.
Повертає anonCount + totalCount (підписники в межах кімнати, включно з анонімними глядачами, яких ми не перераховуємо).

## Parameters

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так | Ідентифікатор URL сторінки (очищується на сервері). |
| afterName | string | query | Ні | Курсор: передайте nextAfterName з попередньої відповіді. |
| afterUserId | string | query | Ні | Курсор-тайбрейкер: передайте nextAfterUserId з попередньої відповіді. Обов'язково, коли встановлено afterName, щоб при однакових іменах записи не випадали. |

## Response

Повертає: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Ідентифікатор URL сторінки (очищується на сервері).
$after_name = 'after_name_example'; // string | Курсор: передайте nextAfterName з попередньої відповіді.
$after_user_id = 'after_user_id_example'; // string | Курсор-тайбрейкер: передайте nextAfterUserId з попередньої відповіді. Обов'язково, коли встановлено afterName, щоб при однакових іменах записи не випадали.

try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]