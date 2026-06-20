---
Масовне информације о корисницима за тенант. За дате userIds, враћа информације за приказ из User / SSOUser.
Користи се од стране видгета коментара да обогати кориснике који су управо појавили путем догађаја присутности.
Нема контекста странице: приватност се примењује поједнако (приватни профили су маскирани).

## Parameters

| Име | Тип | Локација | Захтевано | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | путања | Да |  |
| ids | string | упит | Да | Ид-ови корисника одвојени зарезом. |

## Response

Враћа: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersInfoResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример getUsersInfo'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако желите да користите прилагођени HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, `GuzzleHttp\Client` ће бити коришћен као подразумевани.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$ids = 'ids_example'; // string | Ид-ови корисника одвојени зарезом.

try {
    $result = $apiInstance->getUsersInfo($tenant_id, $ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUsersInfo: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---