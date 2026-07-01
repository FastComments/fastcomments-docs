anmodning
tenantId
afterId

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| afterId | string | query | Nej |  |
| limit | integer | query | Nej |  |
| tags | array | query | Nej |  |
| sso | string | query | Nej |  |
| isCrawler | boolean | query | Nej |  |
| includeUserInfo | boolean | query | Nej |  |

## Svar

Returnerer: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PublicFeedPostsResponse.php)

## Eksempel

[inline-code-attrs-start title = 'getFeedPostsPublic Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Hvis du vil bruge en brugerdefineret http-klient, skal du videregive din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // streng
$options = [
    'after_id' => 'after_id_example', // streng
    'limit' => 56, // int
    'tags' => array('tags_example'), // string[]
    'sso' => 'sso_example', // streng
    'is_crawler' => True, // bool
    'include_user_info' => True, // bool
];


try {
    $result = $apiInstance->getFeedPostsPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Undtagelse ved kald af PublicApi->getFeedPostsPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]