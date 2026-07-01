List pages for a tenant. Used by the FChat desktop client to populate its room list.  
**Ukrainian:** Перелічити сторінки для орендаря. Використовується настільним клієнтом FChat для заповнення списку кімнат.

Requires `enableFChat` to be true on the resolved custom config for each page.  
**Ukrainian:** Потрібно, щоб `enableFChat` було `true` у розв'язаній користувацькій конфігурації для кожної сторінки.

Pages that require SSO are filtered against the requesting user's group access.  
**Ukrainian:** Сторінки, що вимагають SSO, фільтруються згідно з груповим доступом запитуючого користувача.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозорий курсор пагінації, повернений як `nextCursor` з попереднього запиту. Пов'язаний з тим же `sortBy`. |
| limit | integer | query | No | 1..200, за замовчуванням 50 |
| q | string | query | No | Необов'язковий нечутливий до регістру фільтр префікса назви. |
| sortBy | string | query | No | Порядок сортування. `updatedAt` (за замовчуванням, спочатку найновіші), `commentCount` (спочатку з найбільшою кількістю коментарів) або `title` (за алфавітом). |
| hasComments | boolean | query | No | Якщо `true`, повернути лише сторінки, що мають хоча б один коментар. |

## Response

Повертає: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Example

[inline-code-attrs-start title = 'Приклад getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте ваш клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, `GuzzleHttp\Client` буде використаний за замовчуванням.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Непрозорий курсор пагінації, повернений як `nextCursor` з попереднього запиту. Пов'язаний з тим же `sortBy`.
    'limit' => 56, // int | 1..200, за замовчуванням 50
    'q' => 'q_example', // string | Необов'язковий нечутливий до регістру фільтр префікса назви.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Порядок сортування. `updatedAt` (за замовчуванням, спочатку найновіші), `commentCount` (спочатку з найбільшою кількістю коментарів) або `title` (за алфавітом).
    'has_comments' => True, // bool | Якщо `true`, повернути лише сторінки, що мають хоча б один коментар.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Виняток під час виклику PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]