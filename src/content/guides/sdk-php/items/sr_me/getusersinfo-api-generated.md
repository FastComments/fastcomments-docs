Пакет информација о корисницима за закупца. За дате userIds, враћа информације за приказ из User / SSOUser.
Користи га видгет коментара да обогати кориснике који су се управо појавили путем догађаја присутности.
Нема контекста странице: приватност се примењује једнако (приватни профили су маскирани).

## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| ids | string | query | Да | userIds раздвојени зарезима. |

## Одговор

Враћа: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersInfoResponse.php)

## Пример

[inline-code-attrs-start title = 'getUsersInfo Пример'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако желите да користите прилагођени HTTP клијент, проследите клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, као подразумевани ће се користити `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$ids = 'ids_example'; // string | Comma-delimited userIds.

try {
    $result = $apiInstance->getUsersInfo($tenant_id, $ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUsersInfo: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]