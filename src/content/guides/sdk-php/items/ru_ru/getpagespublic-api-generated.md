Выводит список страниц для тенанта. Используется десктоп-клиентом FChat для заполнения списка комнат.
Требует, чтобы `enableFChat` был равен true в resolved custom config для каждой страницы.
Страницы, требующие SSO, фильтруются в соответствии с групповым доступом запрашивающего пользователя.

## Параметры

| Имя | Тип | Расположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| cursor | string | query | Нет | Непрозрачный курсор пагинации, возвращаемый как `nextCursor` из предыдущего запроса. Привязан к тому же `sortBy`. |
| limit | integer | query | Нет | 1..200, по умолчанию 50 |
| q | string | query | Нет | Необязательный фильтр по префиксу заголовка без учета регистра. |
| sortBy | string | query | Нет | Порядок сортировки. `updatedAt` (по умолчанию, сначала новые), `commentCount` (сначала с наибольшим числом комментариев) или `title` (по алфавиту). |
| hasComments | boolean | query | Нет | Если true, возвращать только страницы, у которых есть хотя бы один комментарий. |

## Ответ

Возвращает: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Если вы хотите использовать собственный http-клиент, передайте свой клиент, реализующий `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использоваться `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Непрозрачный курсор пагинации, возвращаемый как `nextCursor` из предыдущего запроса. Привязан к тому же `sortBy`.
    'limit' => 56, // int | 1..200, по умолчанию 50
    'q' => 'q_example', // string | Необязательный фильтр по префиксу заголовка без учета регистра.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Порядок сортировки. `updatedAt` (по умолчанию, сначала новые), `commentCount` (сначала с наибольшим числом комментариев) или `title` (по алфавиту).
    'has_comments' => True, // bool | Если true, возвращать только страницы, у которых есть хотя бы один комментарий.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]