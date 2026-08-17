## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| usernameStartsWith | string | query | No |  |
| mentionGroupIds | array | query | No |  |
| sso | string | query | No |  |
| searchSection | string | query | No |  |

## Одговор

Враћа: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SearchUsersResult.php)

## Пример

[inline-code-attrs-start title = 'searchUsers Primer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако желите да користите прилагођени HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционо, `GuzzleHttp\Client` ће се користити као подразумевано.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // стринг
$url_id = 'url_id_example'; // стринг
$options = [
    'username_starts_with' => 'username_starts_with_example', // стринг
    'mention_group_ids' => array('mention_group_ids_example'), // стринг[]
    'sso' => 'sso_example', // стринг
    'search_section' => 'search_section_example', // стринг
];


try {
    $result = $apiInstance->searchUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->searchUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]