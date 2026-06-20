Потребители, които са коментирали страницата, но в момента НЕ са онлайн. Подредени по displayName.
Използвайте това след като изчерпате /users/online, за да рендерирате секция "Членове".
Курсорна пагинация по commenterName: сървърът обхожда частичния индекс {tenantId, urlId, commenterName}
от afterName напред чрез $gt, без разход за $skip.

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор на URL на страницата (обработен на страната на сървъра). |
| afterName | string | query | Не | Курсор: предайте nextAfterName от предишния отговор. |
| afterUserId | string | query | Не | Курсор за разбиване на равенство: предайте nextAfterUserId от предишния отговор. Изисква се, когато afterName е зададен, за да не се изпуснат записи при еднакви имена. |

## Отговор

Връща: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример за getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако искате да използвате персонализиран HTTP клиент, подайте ваш клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е по избор, `GuzzleHttp\Client` ще бъде използван по подразбиране.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Идентификатор на URL на страницата (обработен на страната на сървъра).
$after_name = 'after_name_example'; // string | Курсор: предайте nextAfterName от предишния отговор.
$after_user_id = 'after_user_id_example'; // string | Курсор за разбиване на равенство: предайте nextAfterUserId от предишния отговор. Изисква се, когато afterName е зададен, за да не се изпуснат записи при еднакви имена.

try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]