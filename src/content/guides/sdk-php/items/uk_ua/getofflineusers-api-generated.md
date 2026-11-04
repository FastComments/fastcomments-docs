Коментатори на сторінці, які наразі НЕ в мережі. Відсортовані за displayName.
Використовуйте це після вичерпання /users/online для відображення розділу «Учасники».
Курсорна пагінація за commenterName: сервер рухається частковим індексом {tenantId, urlId, commenterName}
від afterName вперед за допомогою $gt, без витрат на $skip.

## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так | Ідентифікатор URL сторінки (очищається на сервері). |
| afterName | string | query | Ні | Курсор: передайте nextAfterName з попередньої відповіді. |
| afterUserId | string | query | Ні | Додатковий критерій курсора: передайте nextAfterUserId з попередньої відповіді. Обов'язковий, коли встановлено afterName, щоб збіги імен не втрачали записи. |

## Відповідь

Повертає: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Якщо хочете використовувати власний http-клієнт, передайте свій клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Ідентифікатор URL сторінки (очищається на сервері).
$options = [
    'after_name' => 'after_name_example', // string | Курсор: передайте nextAfterName з попередньої відповіді.
    'after_user_id' => 'after_user_id_example', // string | Додатковий критерій курсора: передайте nextAfterUserId з попередньої відповіді. Обов'язковий, коли встановлено afterName, щоб збіги імен не втрачали записи.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]