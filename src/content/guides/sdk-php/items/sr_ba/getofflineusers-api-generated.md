Претходни коментатори на страници који НИСУ тренутно онлајн. Сортирано по displayName.
Користите ово након исцрпљавања /users/online да бисте приказали "Чланови" секцију.
Курсор пагинација по commenterName: сервер пролази делимични {tenantId, urlId, commenterName}
индекс од afterName унапред преко $gt, без трошка $skip.

## Параметри

| Назив | Тип | Локација | Захтевано | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор URL странице (очишћен на серверској страни). |
| afterName | string | query | Не | Курсор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | Не | Курсор за разрешење везаних случајева: проследите nextAfterUserId из претходног одговора. Обавезно када је afterName подешен да везе по имену не би испустиле уносе. |

## Одговор

Враћа: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Пример

[inline-code-attrs-start title = 'getOfflineUsers Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако желите користити прилагођени HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, `GuzzleHttp\Client` ће бити коришћен по подразумеваној вредности.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Идентификатор URL странице (очишћен на серверској страни).
$after_name = 'after_name_example'; // string | Курсор: проследите nextAfterName из претходног одговора.
$after_user_id = 'after_user_id_example'; // string | Курсор за разрешење везаних случајева: проследите nextAfterUserId из претходног одговора. Обавезно када је afterName подешен да везе по имену не би испустиле уносе.

try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]