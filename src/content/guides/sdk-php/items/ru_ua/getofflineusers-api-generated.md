Прошлые комментаторы на странице, которые В ДАННЫЙ МОМЕНТ НЕ в сети. Отсортировано по displayName.
Используйте это после исчерпания /users/online, чтобы отобразить секцию «Участники».
Пагинация курсором по commenterName: сервер проходит по частичному индексу {tenantId, urlId, commenterName} от afterName вперёд через $gt, без использования $skip.

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор URL страницы (очищается на стороне сервера). |
| afterName | string | query | Нет | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | Нет | Дополнительный параметр курсора (tiebreaker): передайте nextAfterUserId из предыдущего ответа. Требуется, когда установлен afterName, чтобы записи с одинаковыми именами не пропадали. |

## Ответ

Возвращает: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример использования getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Если вы хотите использовать кастомный HTTP-клиент, передайте ваш клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это необязательно, `GuzzleHttp\Client` будет использован по умолчанию.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Идентификатор URL страницы (очищается на стороне сервера).
$after_name = 'after_name_example'; // string | Курсор: передайте nextAfterName из предыдущего ответа.
$after_user_id = 'after_user_id_example'; // string | Дополнительный параметр курсора (tiebreaker): передайте nextAfterUserId из предыдущего ответа. Требуется, когда установлен afterName, чтобы записи с одинаковыми именами не пропадали.

try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]