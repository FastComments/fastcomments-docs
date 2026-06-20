Сейчас онлайн зрители страницы: люди, чья websocket-сессия в данный момент подписана на страницу.
Возвращает anonCount + totalCount (подписчики всей комнаты, включая анонимных зрителей, которых мы не перечисляем).

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор URL страницы (очищается на стороне сервера). |
| afterName | string | query | Нет | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | Нет | Тайбрейкер курсора: передайте nextAfterUserId из предыдущего ответа. Обязательно, когда установлен afterName, чтобы совпадения по имени не приводили к пропуску записей. |

## Ответ

Возвращает: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Если вы хотите использовать пользовательский HTTP-клиент, передайте ваш клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использоваться `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Идентификатор URL страницы (очищается на стороне сервера).
$after_name = 'after_name_example'; // string | Курсор: передайте nextAfterName из предыдущего ответа.
$after_user_id = 'after_user_id_example'; // string | Тайбрейкер курсора: передайте nextAfterUserId из предыдущего ответа. Обязательно, когда установлен afterName, чтобы совпадения по имени не приводили к пропуску записей.

try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]