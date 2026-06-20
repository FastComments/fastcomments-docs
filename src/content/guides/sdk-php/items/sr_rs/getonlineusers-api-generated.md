Тренутно-приказани посматрачи странице: људи чија je websocket сесија тренутно претплаћена на страницу.
Враћа anonCount + totalCount (претплатници у оквиру собе, укључујући анонимне посматраче које не набрајамо).

## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL странице (очишћен на серверу). |
| afterName | string | query | No | Курсор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | No | Курсор за разрешење везе: проследите nextAfterUserId из претходног одговора. Обавезно када је afterName подешен како везе по имену не би довеле до изостављања уноса. |

## Одговор

Враћа: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример за getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако желите да користите прилагођени http клијент, проследите свој клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, као подразумевани ће се користити `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Идентификатор URL странице (очишћен на серверу).
$after_name = 'after_name_example'; // string | Курсор: проследите nextAfterName из претходног одговора.
$after_user_id = 'after_user_id_example'; // string | Курсор за разрешење везе: проследите nextAfterUserId из претходног одговора. Обавезно када је afterName подешен како везе по имену не би довеле до изостављања уноса.

try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]