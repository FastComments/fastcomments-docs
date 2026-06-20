Изброява страници за tenant. Използва се от настолния клиент FChat за попълване на списъка с чат стаи. Изисква `enableFChat` да е true в изчислената персонализирана конфигурация за всяка страница. Страниците, които изискват SSO, се филтрират според груповия достъп на заявяващия потребител.

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозрачен курсор за странициране, върнат като `nextCursor` от предишна заявка. Свързан е със същия `sortBy`. |
| limit | integer | query | No | 1..200, по подразбиране 50 |
| q | string | query | No | Незадължителен филтър за префикс на заглавие, нечувствителен към регистъра. |
| sortBy | string | query | No | Ред на сортиране. `updatedAt` (по подразбиране — най-новите първи), `commentCount` (страниците с най-много коментари първи), или `title` (азбучно). |
| hasComments | boolean | query | No | Ако е true, връща само страници с поне един коментар. |

## Отговор

Връща: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример за getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако искате да използвате собствен HTTP клиент, предайте вашия клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е по избор; по подразбиране ще бъде използван `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | Непрозрачен курсор за странициране, върнат като `nextCursor` от предишна заявка. Свързан е със същия `sortBy`.
$limit = 56; // int | 1..200, по подразбиране 50
$q = 'q_example'; // string | Незадължителен филтър за префикс на заглавие, нечувствителен към регистъра.
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | Ред на сортиране. `updatedAt` (по подразбиране — най-новите първи), `commentCount` (страниците с най-много коментари първи), или `title` (азбучно).
$has_comments = True; // bool | Ако е true, връща само страници с поне един коментар.

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]