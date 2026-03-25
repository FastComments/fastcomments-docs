## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |
| usernameStartsWith | string | query | Не |  |
| mentionGroupIds | array | query | Не |  |
| sso | string | query | Не |  |
| searchSection | string | query | Не |  |

## Одговор

Враћа: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SearchUsers200Response.php)

## Пример

[inline-code-attrs-start title = 'Пример за searchUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако желите да користите прилагођен HTTP клијент, проследите свој клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, подразумевано ће бити коришћен `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$username_starts_with = 'username_starts_with_example'; // string
$mention_group_ids = array('mention_group_ids_example'); // string[]
$sso = 'sso_example'; // string
$search_section = 'search_section_example'; // string

try {
    $result = $apiInstance->searchUsers($tenant_id, $url_id, $username_starts_with, $mention_group_ids, $sso, $search_section);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->searchUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]