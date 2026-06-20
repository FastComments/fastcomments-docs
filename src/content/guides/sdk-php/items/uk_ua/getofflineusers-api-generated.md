Колишні коментатори на сторінці, які наразі не в мережі. Відсортовано за displayName.
Використовуйте це після вичерпання /users/online, щоб відобразити секцію «Учасники».
Посторінкова пагінація курсором за commenterName: сервер обходить частковий {tenantId, urlId, commenterName}
індекс від afterName вперед через $gt, без витрат на $skip.

## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Page URL identifier (очищується на сервері). |
| afterName | string | query | No | Курсор: передайте nextAfterName з попередньої відповіді. |
| afterUserId | string | query | No | Розв'язувач нічиїх для курсора: передайте nextAfterUserId з попередньої відповіді. Потрібно, коли afterName встановлено, щоб записи з однаковими іменами не були пропущені. |

## Відповідь

Повертає: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Page URL identifier (очищується на сервері).
$after_name = 'after_name_example'; // string | Курсор: передайте nextAfterName з попередньої відповіді.
$after_user_id = 'after_user_id_example'; // string | Розв'язувач нічиїх для курсора: передайте nextAfterUserId з попередньої відповіді. Потрібно, коли afterName встановлено, щоб записи з однаковими іменами не були пропущені.

try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]