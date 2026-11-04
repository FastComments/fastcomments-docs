Минали коментатори на страницата, които в момента НЕ са онлайн. Подредени по displayName.
Използвайте това след изчерпване на /users/online, за да визуализирате секция „Членове“.
Пагинацията с курсор е по commenterName: сървърът обходява частичния индекс {tenantId, urlId, commenterName} от afterName напред чрез $gt, без разход за $skip.

## Параметри

| Име | Тип | Местоположение | Задължителен | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор на URL на страницата (почиства се от сървъра). |
| afterName | string | query | Не | Курсор: подайте nextAfterName от предишния отговор. |
| afterUserId | string | query | Не | Допълнителен критерий за курса: подайте nextAfterUserId от предишния отговор. Задължителен, когато afterName е зададен, така че съвпаденията по име да не пропуснат записи. |

## Отговор

Връща: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример за getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако искате да използвате персонализиран http клиент, подайте свой клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е по избор, `GuzzleHttp\Client` ще се използва по подразбиране.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Идентификатор на URL на страницата (почиства се от сървъра).
$options = [
    'after_name' => 'after_name_example', // string | Курсор: подайте nextAfterName от предишния отговор.
    'after_user_id' => 'after_user_id_example', // string | Допълнителен критерий за курса: подайте nextAfterUserId от предишния отговор. Задължителен, когато afterName е зададен, така че съвпаденията по име да не пропуснат записи.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]