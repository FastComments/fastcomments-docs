Список страниц для тенанта. Используется десктоп-клиентом FChat для заполнения его списка комнат.
Требуется, чтобы `enableFChat` был true в итоговой пользовательской конфигурации для каждой страницы.
Страницы, требующие SSO, фильтруются в соответствии с групповым доступом запрашивающего пользователя.

## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозрачный курсор пагинации, возвращаемый как `nextCursor` в предыдущем запросе. Связан с тем же `sortBy`. |
| limit | integer | query | No | 1..200, по умолчанию 50 |
| q | string | query | No | Необязательный регистронезависимый фильтр по префиксу заголовка. |
| sortBy | string | query | No | Порядок сортировки. `updatedAt` (по умолчанию, сначала новые), `commentCount` (сначала с наибольшим числом комментариев), или `title` (по алфавиту). |
| hasComments | boolean | query | No | Если true, возвращать только страницы с как минимум одним комментарием. |

## Ответ

Возвращает: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Если вы хотите использовать пользовательский HTTP-клиент, передайте ваш клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | Непрозрачный курсор пагинации, возвращаемый как `nextCursor` в предыдущем запросе. Связан с тем же `sortBy`.
$limit = 56; // int | 1..200, по умолчанию 50
$q = 'q_example'; // string | Необязательный регистронезависимый фильтр по префиксу заголовка.
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | Порядок сортировки. `updatedAt` (по умолчанию, сначала новые), `commentCount` (сначала с наибольшим числом комментариев), или `title` (по алфавиту).
$has_comments = True; // bool | Если true, возвращать только страницы с как минимум одним комментарием.

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]