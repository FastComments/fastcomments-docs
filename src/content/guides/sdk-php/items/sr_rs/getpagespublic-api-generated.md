---
Листа страница за тенанта. Користи се од стране FChat десктоп клијента за попуњавање листе соба.
Захтева да је `enableFChat` постављен на true у решеном прилагођеном конфигу за сваку страницу.
Странице које захтевају SSO филтрирају се у складу са приступом групе корисника који тражи.

## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| cursor | string | query | Не | Нечитљив курсор пагинације који се враћа као `nextCursor` из претходног захтева. Повезан са истим `sortBy`. |
| limit | integer | query | Не | 1..200, подразумевано 50 |
| q | string | query | Не | Опционални филтер префикса наслова који није осетљив на велика/мала слова. |
| sortBy | string | query | Не | Редослед сортирања. `updatedAt` (подразумевано, најновије прве), `commentCount` (странице са највише коментара прве), или `title` (азбучно). |
| hasComments | boolean | query | Не | Ако је true, враћају се само странице са најмање једним коментаром. |

## Одговор

Враћа: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Пример

[inline-code-attrs-start title = 'getPagesPublic Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако желите да користите прилагођени http клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, као подразумевани ће бити коришћен `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | Нечитљив курсор пагинације који се враћа као `nextCursor` из претходног захтева. Повезан са истим `sortBy`.
$limit = 56; // int | 1..200, подразумевано 50
$q = 'q_example'; // string | Опционални филтер префикса наслова који није осетљив на велика/мала слова.
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | Редослед сортирања. `updatedAt` (подразумевано, најновије прве), `commentCount` (странице са највише коментара прве), или `title` (азбучно).
$has_comments = True; // bool | Ако је true, враћају се само странице са најмање једним коментаром.

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---