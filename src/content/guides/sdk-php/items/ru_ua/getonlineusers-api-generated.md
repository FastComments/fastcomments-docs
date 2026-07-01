Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
Текущие онлайн‑просмотры страницы: пользователи, чья websocket‑сессия в данный момент подписана на страницу.

Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).  
Возвращает anonCount + totalCount (подписчики на уровне комнаты, включая анонимных зрителей, которые не перечисляются).

## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL страницы (очищенный на сервере). |
| afterName | string | query | No | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | No | Тайбрейкер курсора: передайте nextAfterUserId из предыдущего ответа. Требуется, когда установлен afterName, чтобы записи с одинаковыми именами не отбрасывались. |

## Ответ

Возвращает: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Если вы хотите использовать кастомный http‑клиент, передайте ваш клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это опционально, по умолчанию будет использоваться `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Идентификатор URL страницы (очищенный на сервере).
$options = [
    'after_name' => 'after_name_example', // string | Курсор: передайте nextAfterName из предыдущего ответа.
    'after_user_id' => 'after_user_id_example', // string | Тайбрейкер курсора: передайте nextAfterUserId из предыдущего ответа. Требуется, когда установлен afterName, чтобы записи с одинаковыми именами не отбрасывались.
];


try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]