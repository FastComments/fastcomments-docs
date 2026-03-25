## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| usernameStartsWith | string | query | Nej |  |
| mentionGroupIds | array | query | Nej |  |
| sso | string | query | Nej |  |
| searchSection | string | query | Nej |  |

## Svar

Returnerer: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SearchUsers200Response.php)

## Eksempel

[inline-code-attrs-start title = 'searchUsers Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, skal du aflevere din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
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