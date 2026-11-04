Прошли коментари на страници који ТРЕНУТНО НИСУ на мрежи. Сортирани по displayName.
Користите ово након што исцрпите /users/online за приказ секције „Чланови".
Cursor пагинација по commenterName: сервер пролази кроз делимични {tenantId, urlId, commenterName}
индекс од afterName напред преко $gt, без трошка $skip.

## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор URL-а странице (очишћен на серверу). |
| afterName | string | query | Не | Cursor: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | Не | Cursor везивач: проследите nextAfterUserId из претходног одговора. Обавезно када је afterName постављен, да уноси са истим именом не би нестанули. |

## Одговор

Враћа: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## Пример

[inline-code-attrs-start title = 'getOfflineUsers пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако желите да користите прилагођени http клијент, проследите свој клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, `GuzzleHttp\Client` ће бити коришћен као подразумевани.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Идентификатор URL-а странице (очишћен на серверу).
$options = [
    'after_name' => 'after_name_example', // string | Cursor: проследите nextAfterName из претходног одговора.
    'after_user_id' => 'after_user_id_example', // string | Cursor везивач: проследите nextAfterUserId из претходног одговора. Обавезно када је afterName постављен, да уноси са истим именом не би нестанули.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]