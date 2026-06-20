Претходни коментатори на страници који тренутно нису на мрежи. Сортирано по displayName.
Користите ово након што исцрпите /users/online да бисте приказали "Members" секцију.
Курсорска пагинација по commenterName: сервер пролази делимични индекс {tenantId, urlId, commenterName} од afterName унапред користећи $gt, без трошкова $skip.

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL странице (очишћен на серверу). |
| afterName | string | query | No | Курсор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | No | Курсор за разрешење везе: проследите nextAfterUserId из претходног одговора. Захтевано када је afterName подешен тако да везе по имену не испусте уносе. |

## Одговор

Враћа: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако желите да користите прилагођени HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционо, подразумевано ће бити коришћен `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Идентификатор URL странице (очишћен на серверу).
$after_name = 'after_name_example'; // string | Курсор: проследите nextAfterName из претходног одговора.
$after_user_id = 'after_user_id_example'; // string | Курсор за разрешење везе: проследите nextAfterUserId из претходног одговора. Обавезно када је afterName подешен, како би везе по имену не испустиле уносе.

try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]