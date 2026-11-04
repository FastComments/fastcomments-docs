Списъци със страници за наемател. Използва се от FChat десктоп клиента за попълване на списъка му със стаи.
Изисква `enableFChat` да е true в резолвнатата персонализирана конфигурация за всяка страница.
Страниците, изискващи SSO, се филтрират спрямо груповия достъп на заявяващия потребител.

## Параметри

| Име | Тип | Местоположение | Задължителен | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| cursor | string | query | Не | Непрозрачен курсор за пагинация, върнат като `nextCursor` от предишна заявка. Обвързан със същия `sortBy`. |
| limit | integer | query | Не | 1..200, по подразбиране 50 |
| q | string | query | Не | Незадължителен филтър по префикс на заглавие, без разлика в регистъра. |
| sortBy | string | query | Не | Ред на сортиране. `updatedAt` (по подразбиране, най-новите първо), `commentCount` (най-много коментари първо) или `title` (по азбучен ред). |
| hasComments | boolean | query | Не | Ако е true, връща само страници с поне един коментар. |

## Отговор

Връща: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример за getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако искате да използвате персонализиран http клиент, подайте своя клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е незадължително, `GuzzleHttp\Client` ще се използва по подразбиране.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Непрозрачен курсор за пагинация, върнат като `nextCursor` от предишна заявка. Обвързан със същия `sortBy`.
    'limit' => 56, // int | 1..200, по подразбиране 50
    'q' => 'q_example', // string | Незадължителен филтър по префикс на заглавие, без разлика в регистъра.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Ред на сортиране. `updatedAt` (по подразбиране, най-новите първо), `commentCount` (най-много коментари първо) или `title` (по азбучен ред).
    'has_comments' => True, // bool | Ако е true, връща само страници с поне един коментар.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]