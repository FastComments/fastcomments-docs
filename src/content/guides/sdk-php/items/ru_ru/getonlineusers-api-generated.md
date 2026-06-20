В настоящее время онлайн-зрители страницы: люди, чья WebSocket-сессия в данный момент подписана на эту страницу.
Возвращает anonCount + totalCount (подписчики комнаты в целом, включая анонимных зрителей, которых мы не перечисляем).

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL страницы (очищается на сервере). |
| afterName | string | query | No | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | No | Дополнительный курсор (тайбрейкер): передайте nextAfterUserId из предыдущего ответа. Обязательно, когда задан afterName, чтобы записи с одинаковыми именами не отбрасывались. |

## Ответ

Возвращает: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Если вы хотите использовать пользовательский HTTP-клиент, передайте клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Идентификатор URL страницы (очищается на сервере).
$after_name = 'after_name_example'; // string | Курсор: передайте nextAfterName из предыдущего ответа.
$after_user_id = 'after_user_id_example'; // string | Дополнительный курсор (тайбрейкер): передайте nextAfterUserId из предыдущего ответа. Обязательно, когда задан afterName, чтобы записи с одинаковыми именами не отбрасывались.

try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]