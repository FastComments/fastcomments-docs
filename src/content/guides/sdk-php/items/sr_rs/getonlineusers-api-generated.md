Тренутно онлајн посетиоци странице: људи чија вебсокет сесија је тренутно претплаћена на страницу.  
Враћа anonCount + totalCount (претплатници у соби, укључујући анонимне посетиоце које не евидентирамо).

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL странице (очишћен на серверу). |
| afterName | string | query | No | Курсор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | No | Тијебрејкер за курзор: проследите nextAfterUserId из претходног одговора. Обавезно када је afterName постављен како се не би изгубили уноси при везаним именима. |

## Одговор

Враћа: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Пример

[inline-code-attrs-start title = 'Primer getOnlineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако желите да користите прилагођени HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционо, `GuzzleHttp\Client` ће се користити као подразумевано.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | Идентификатор URL странице (очишћен на серверу).
$options = [
    'after_name' => 'after_name_example', // string | Курсор: проследите nextAfterName из претходног одговора.
    'after_user_id' => 'after_user_id_example', // string | Тијебрејкер за курзор: проследите nextAfterUserId из претходног одговора. Обавезно када је afterName постављен како се не би изгубили уноси при везаним именима.
];


try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---