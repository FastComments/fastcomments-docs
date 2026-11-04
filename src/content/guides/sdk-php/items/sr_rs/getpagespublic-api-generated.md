Наводи странице за закупца (tenant). Користи FChat десктоп клијент да попуни своју листу соба.
Захтева да `enableFChat` буде true на резолвованој прилагођеној конфигурацији за сваку страницу.
Странице које захтевају SSO се филтрирају у односу на приступ групама тражеоца.

## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| cursor | string | query | Не | Непрозирни пагинациони курсор враћен као `nextCursor` из претходног захтева. Везан за исти `sortBy`. |
| limit | integer | query | Не | 1..200, подразумевано 50 |
| q | string | query | Не | Опциони филтер префикса наслова, неосетљив на велика и мала слова. |
| sortBy | string | query | Не | Редослед сортирања. `updatedAt` (подразумевано, најновије прво), `commentCount` (највише коментара прво) или `title` (абецедно). |
| hasComments | boolean | query | Не | Ако је true, враћа само странице са бар једним коментаром. |

## Одговор

Враћа: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Пример

[inline-code-attrs-start title = 'getPagesPublic пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако желите да користите прилагођени http клијент, проследите свој клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, `GuzzleHttp\Client` ће бити коришћен као подразумевани.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Непрозирни пагинациони курсор враћен као `nextCursor` из претходног захтева. Везан за исти `sortBy`.
    'limit' => 56, // int | 1..200, подразумевано 50
    'q' => 'q_example', // string | Опциони филтер префикса наслова, неосетљив на велика и мала слова.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Редослед сортирања. `updatedAt` (подразумевано, најновије прво), `commentCount` (највише коментара прво) или `title` (абецедно).
    'has_comments' => True, // bool | Ако је true, враћа само странице са бар једним коментаром.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]