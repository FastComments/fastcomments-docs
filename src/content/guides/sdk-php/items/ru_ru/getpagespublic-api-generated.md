List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозрачный курсор пагинации, возвращённый как `nextCursor` из предыдущего запроса. Связан с тем же `sortBy`. |
| limit | integer | query | No | 1..200, default 50 |
| q | string | query | No | Необязательный регистронезависимый фильтр префикса заголовка. |
| sortBy | string | query | No | Порядок сортировки. `updatedAt` (по умолчанию, новейшие первыми), `commentCount` (сначала страницы с наибольшим числом комментариев) или `title` (в алфавитном порядке). |
| hasComments | boolean | query | No | Если true, возвращаются только страницы, содержащие хотя бы один комментарий. |

## Ответ

Возвращает: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



// $apiInstance = new FastComments\Client\Api\PublicApi(
    // Если вы хотите использовать собственный HTTP‑клиент, передайте свой клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Непрозрачный курсор пагинации, возвращённый как `nextCursor` из предыдущего запроса. Связан с тем же `sortBy`.
    'limit' => 56, // int | 1..200, default 50
    'q' => 'q_example', // string | Необязательный регистронезависимый фильтр префикса заголовка.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Порядок сортировки. `updatedAt` (по умолчанию, новейшие первыми), `commentCount` (сначала страницы с наибольшим числом комментариев) или `title` (в алфавитном порядке).
    'has_comments' => True, // bool | Если true, возвращаются только страницы, содержащие хотя бы один комментарий.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---