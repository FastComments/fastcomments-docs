Листа страница за тенанта. Користи се у FChat десктоп клијенту за попуњавање његовог списка просторија.
Потребно је да `enableFChat` буде true у решеној прилагођеној конфигурацији за сваку страницу.
Странице које захтијевају SSO филтрирају се према приступу група корисника који прави захтјев.

## Parameters

| Назив | Тип | Локација | Потребно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| cursor | string | query | Не | Непрозирни курсор пагинације враћен као `nextCursor` из претходног захтјева. Везан за исти `sortBy`. |
| limit | integer | query | Не | 1..200, подразумевано 50 |
| q | string | query | Не | Опционо, филтер префикса наслова који није осетљив на регистар. |
| sortBy | string | query | Не | Редослијед сортирања. `updatedAt` (подразумевано, најновије прво), `commentCount` (највише коментара прво), или `title` (азбучно). |
| hasComments | boolean | query | Не | Ако је true, враћају се само странице са најмање једним коментаром. |

## Response

Враћа: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Пример

[inline-code-attrs-start title = 'getPagesPublic Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако желите користити прилагођени HTTP клијент, прослиједите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, користиће се `GuzzleHttp\Client` као подразумевани.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | Непрозирни курсор пагинације враћен као `nextCursor` из претходног захтјева. Везан за исти `sortBy`.
$limit = 56; // int | 1..200, подразумевано 50
$q = 'q_example'; // string | Опционо, филтер префикса наслова који није осетљив на регистар.
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | Редослијед сортирања. `updatedAt` (подразумевано, најновије прво), `commentCount` (највише коментара прво), или `title` (азбучно).
$has_comments = True; // bool | Ако је true, враћају се само странице са најмање једним коментаром.

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]