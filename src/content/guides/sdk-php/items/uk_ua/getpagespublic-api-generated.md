Список сторінок для орендаря. Використовується настільним клієнтом FChat для заповнення списку кімнат.
Потребує, щоб `enableFChat` мав значення true у визначеній користувацькій конфігурації для кожної сторінки.
Сторінки, що вимагають SSO, фільтруються за доступом групи користувача, що робить запит.

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозорий курсор пагінації, повернений як `nextCursor` з попереднього запиту. Прив'язаний до того самого `sortBy`. |
| limit | integer | query | No | 1..200, за замовчуванням 50 |
| q | string | query | No | Необов'язковий фільтр префіксу заголовка без урахування регістру. |
| sortBy | string | query | No | Порядок сортування. `updatedAt` (за замовчуванням, найновіші першими), `commentCount` (найбільше коментарів першими), або `title` (алфавітний). |
| hasComments | boolean | query | No | Якщо true, повертати лише сторінки з щонайменше одним коментарем. |

## Відповідь

Повертає: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте ваш клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це опційно, як замовчування буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | Непрозорий курсор пагінації, повернений як `nextCursor` з попереднього запиту. Прив'язаний до того самого `sortBy`.
$limit = 56; // int | 1..200, за замовчуванням 50
$q = 'q_example'; // string | Необов'язковий фільтр префіксу заголовка без урахування регістру.
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | Порядок сортування. `updatedAt` (за замовчуванням, найновіші першими), `commentCount` (найбільше коментарів першими), або `title` (алфавітний).
$has_comments = True; // bool | Якщо true, повертати лише сторінки з щонайменше одним коментарем.

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]