---
Текущие онлайн‑просмотры страницы: люди, чья websocket‑сессия подписана на страницу в данный момент.  
Возвращает anonCount + totalCount (подписчики по всей комнате, включая анонимных зрителей, которых мы не перечисляем).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL страницы (очищен на сервере). |
| afterName | string | query | No | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | No | Тайбрейкер курсора: передайте nextAfterUserId из предыдущего ответа. Требуется, когда afterName установлен, чтобы привязанные к имени записи не терялись. |

## Response

Возвращает: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Example

[inline-code-attrs-start title = 'Пример getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Если вы хотите использовать пользовательский HTTP‑клиент, передайте ваш клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это опционально, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Идентификатор URL страницы (очищен на сервере).
$options = [
    'after_name' => 'after_name_example', // string | Курсор: передайте nextAfterName из предыдущего ответа.
    'after_user_id' => 'after_user_id_example', // string | Тайбрейтер курсора: передайте nextAfterUserId из предыдущего ответа. Требуется, когда afterName установлен, чтобы привязанные к имени записи не терялись.
];


try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---