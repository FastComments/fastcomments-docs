List pages for a tenant. Used by the FChat desktop client to populate its room list.
Requires `enableFChat` to be true on the resolved custom config for each page.
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозорий курсор пагінації, що повертається як `nextCursor` зі попереднього запиту. Прив’язаний до того ж `sortBy`. |
| limit | integer | query | No | 1..200, за замовчуванням 50 |
| q | string | query | No | Необов’язковий нечутливий до регістру фільтр за префіксом назви. |
| sortBy | string | query | No | Порядок сортування. `updatedAt` (за замовчуванням, новіші спочатку), `commentCount` (спочатку з найбільшою кількістю коментарів) або `title` (за алфавітом). |
| hasComments | boolean | query | No | Якщо true, повернути лише сторінки, які мають щонайменше один коментар. |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Example

[inline-code-attrs-start title = 'Приклад getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Якщо ви хочете використати власний HTTP‑клієнт, передайте ваш клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов’язково, за замовчуванням використовується `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // рядок
$options = [
    'cursor' => 'cursor_example', // string | Непрозорий курсор пагінації, що повертається як `nextCursor` зі попереднього запиту. Прив’язаний до того ж `sortBy`.
    'limit' => 56, // int | 1..200, за замовчуванням 50
    'q' => 'q_example', // string | Необов’язковий нечутливий до регістру фільтр за префіксом назви.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Порядок сортування. `updatedAt` (за замовчуванням, новіші спочатку), `commentCount` (спочатку з найбільшою кількістю коментарів) або `title` (за алфавітом).
    'has_comments' => True, // bool | Якщо true, повернути лише сторінки, які мають щонайменше один коментар.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]