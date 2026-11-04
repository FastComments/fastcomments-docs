Перелічує сторінки для тенанта. Використовується десктопним клієнтом FChat для заповнення списку кімнат.
Потребує `enableFChat` = true у вирішеному кастомному конфізі для кожної сторінки.
Сторінки, що вимагають SSO, фільтруються відповідно до групового доступу користувача, який виконує запит.

## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| cursor | string | query | Ні | Непрозорий курсор пагінації, повернутий як `nextCursor` з попереднього запиту. Прив'язаний до того самого `sortBy`. |
| limit | integer | query | Ні | 1..200, за замовчуванням 50 |
| q | string | query | Ні | Необов'язковий фільтр за префіксом назви без урахування регістру. |
| sortBy | string | query | Ні | Порядок сортування. `updatedAt` (за замовчуванням, спочатку новіші), `commentCount` (спочатку з найбільшою кількістю коментарів) або `title` (за алфавітом). |
| hasComments | boolean | query | Ні | Якщо true, повертати лише сторінки, що мають хоча б один коментар. |

## Відповідь

Повертає: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Якщо ви хочете використовувати власний http клієнт, передайте клієнт, що реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Непрозорий курсор пагінації, повернутий як `nextCursor` з попереднього запиту. Прив'язаний до того самого `sortBy`.
    'limit' => 56, // int | 1..200, за замовчуванням 50
    'q' => 'q_example', // string | Необов'язковий фільтр за префіксом назви без урахування регістру.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Порядок сортування. `updatedAt` (за замовчуванням, спочатку новіші), `commentCount` (спочатку з найбільшою кількістю коментарів) або `title` (за алфавітом).
    'has_comments' => True, // bool | Якщо true, повертати лише сторінки, що мають хоча б один коментар.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]