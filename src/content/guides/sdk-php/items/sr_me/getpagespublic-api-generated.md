Набраја странице за тенанта. Користи се од стране FChat десктоп клијента за попуњавање његове листе соба.
Захтева да у решеном прилагођеном конфигу за сваку страницу `enableFChat` буде true.
Странице које захтијевају SSO филтриране су у складу са приступом групе корисника који шаље захтев.

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозирни курсор пагинације који се враћа као `nextCursor` из претходног захтјева. Везан за исти `sortBy`. |
| limit | integer | query | No | 1..200, подразумевано 50 |
| q | string | query | No | Опционо, филтер префикса наслова који није осетљив на велика/мала слова. |
| sortBy | string | query | No | Редослијед сортирања. `updatedAt` (подразумевано, најновије прво), `commentCount` (странице са највише коментара прво), или `title` (абецедно). |
| hasComments | boolean | query | No | Ако је true, враћају се само странице са најмање једним коментаром. |

## Одговор

Враћа: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Пример

[inline-code-attrs-start title = 'getPagesPublic Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако желите да користите прилагођени http клијент, проследите свој клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, `GuzzleHttp\Client` ће бити коришћен као подразумевани.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | Непрозирни курсор пагинације који се враћа као `nextCursor` из претходног захтева. Везан за исти `sortBy`.
$limit = 56; // int | 1..200, подразумевано 50
$q = 'q_example'; // string | Опционо, филтер префикса наслова који није осетљив на велика/мала слова.
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | Редослијед сортирања. `updatedAt` (подразумевано, најновије прво), `commentCount` (странице са највише коментара прво), или `title` (абецедно).
$has_comments = True; // bool | Ако је true, враћају се само странице са најмање једним коментаром.

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]