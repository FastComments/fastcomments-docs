Обимне информације о корисницима за станара (tenant). За дате userIds, враћа информације о приказу из User / SSOUser.
Користи га виџет за коментаре да обогати кориснике који су управо се појавили преко догађаја присуства (presence event).
Без контекста странице: приватност се спроводи једнолико (приватни профили су маскирани).

## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| ids | string | query | Да | userIds раздвојени зарезима. |

## Одговор

Враћа: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersInfoResponse.php)

## Пример

[inline-code-attrs-start title = 'getUsersInfo пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако желите да користите прилагођени http клијент, проследите свој клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционо, `GuzzleHttp\Client` ће се користити као подразумевани.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$ids = 'ids_example'; // string | userIds раздвојени зарезима.


try {
    $result = $apiInstance->getUsersInfo($tenant_id, $ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUsersInfo: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]