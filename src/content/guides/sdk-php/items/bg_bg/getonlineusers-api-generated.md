Онлайн в момента зрители на страница: хора, чиито websocket сесии са абонирани за страницата в момента.
Връща anonCount + totalCount (абонати за стаята като цяло, включително анонимни зрители, които не изброяваме).

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор на URL на страницата (почистен от страна на сървъра). |
| afterName | string | query | Не | Курсор: подайте nextAfterName от предишния отговор. |
| afterUserId | string | query | Не | Курсор за допълнително разпознаване при равенство: подайте nextAfterUserId от предишния отговор. Изисква се когато afterName е зададено, за да не се изпускат записи при еднакви имена. |

## Отговор

Връща: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример за getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако искате да използвате персонализиран HTTP клиент, подайте вашия клиент, който имплементира `GuzzleHttp\ClientInterface`.
    // Това е по избор, по подразбиране ще се използва `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Идентификатор на URL на страницата (почистен от страна на сървъра).
$after_name = 'after_name_example'; // string | Курсор: подайте nextAfterName от предишния отговор.
$after_user_id = 'after_user_id_example'; // string | Курсор за допълнително разпознаване при равенство: подайте nextAfterUserId от предишния отговор. Изисква се когато afterName е зададено, за да не се изпускат записи при еднакви имена.

try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]