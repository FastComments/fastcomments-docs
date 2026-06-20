Перечисляет страницы для тенанта. Используется десктоп-клиентом FChat для заполнения списка комнат.
Требует, чтобы в разрешённой пользовательской конфигурации для каждой страницы `enableFChat` имел значение true.
Страницы, требующие SSO, фильтруются с учётом группового доступа запрашивающего пользователя.

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| cursor | string | query | Нет | Непрозрачный курсор постраничной навигации, возвращённый как `nextCursor` в предыдущем запросе. Привязан к тому же `sortBy`. |
| limit | integer | query | Нет | 1..200, по умолчанию 50 |
| q | string | query | Нет | Необязательный регистронезависимый фильтр по префиксу заголовка. |
| sortBy | string | query | Нет | Порядок сортировки. `updatedAt` (по умолчанию, сначала новейшие), `commentCount` (сначала страницы с наибольшим количеством комментариев), или `title` (в алфавитном порядке). |
| hasComments | boolean | query | Нет | Если true, возвращать только страницы с хотя бы одним комментарием. |

## Ответ

Возвращает: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Если вы хотите использовать собственный HTTP-клиент, передайте клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | Непрозрачный курсор постраничной навигации, возвращённый как `nextCursor` в предыдущем запросе. Привязан к тому же `sortBy`.
$limit = 56; // int | 1..200, по умолчанию 50
$q = 'q_example'; // string | Необязательный регистронезависимый фильтр по префиксу заголовка.
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | Порядок сортировки. `updatedAt` (по умолчанию, сначала новейшие), `commentCount` (сначала страницы с наибольшим количеством комментариев), или `title` (в алфавитном порядке).
$has_comments = True; // bool | Если true, возвращать только страницы с хотя бы одним комментарием.

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]