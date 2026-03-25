## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| usernameStartsWith | string | query | Nee |  |
| mentionGroupIds | array | query | Nee |  |
| sso | string | query | Nee |  |
| searchSection | string | query | Nee |  |

## Antwoord

Retourneert: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SearchUsers200Response.php)

## Voorbeeld

[inline-code-attrs-start title = 'searchUsers Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Als u een aangepaste http-client wilt gebruiken, geef uw client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel, `GuzzleHttp\Client` zal standaard worden gebruikt.
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