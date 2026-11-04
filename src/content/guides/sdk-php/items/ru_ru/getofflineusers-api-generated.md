Прошлые комментаторы страницы, которые НЕ находятся на сайте в данный момент. Отсортированы по displayName.
Используйте этот метод после исчерпания /users/online для отображения раздела «Участники».
Курсорная пагинация по commenterName: сервер проходит по частичному индексу {tenantId, urlId, commenterName}
от afterName вперёд через $gt, без затрат на $skip.

## Параметры

| Имя | Тип | Расположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор URL страницы (очищается на сервере). |
| afterName | string | query | Нет | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | Нет | Дополнительный ключ курсора: передайте nextAfterUserId из предыдущего ответа. Обязателен, когда задан afterName, чтобы записи с одинаковыми именами не терялись. |

## Ответ

Возвращает: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Если вы хотите использовать собственный http-клиент, передайте клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использоваться `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Идентификатор URL страницы (очищается на сервере).
$options = [
    'after_name' => 'after_name_example', // string | Курсор: передайте nextAfterName из предыдущего ответа.
    'after_user_id' => 'after_user_id_example', // string | Дополнительный ключ курсора: передайте nextAfterUserId из предыдущего ответа. Обязателен, когда задан afterName, чтобы записи с одинаковыми именами не терялись.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]