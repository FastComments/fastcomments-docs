## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|--------------|
| tenantId | string | query | Ja |  |
| page | number | query | Nee |  |
| count | number | query | Nee |  |
| text-search | string | query | Nee |  |
| byIPFromComment | string | query | Nee |  |
| filters | string | query | Nee |  |
| searchFilters | string | query | Nee |  |
| sorts | string | query | Nee |  |
| demo | boolean | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPIGetCommentsResponse.php)

## Voorbeeld

[inline-code-attrs-start title = 'getApiComments Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Als je een aangepaste http-client wilt gebruiken, geef je client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel, `GuzzleHttp\Client` wordt als standaard gebruikt.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'page' => 3.4, // float
    'count' => 3.4, // float
    'text_search' => 'text_search_example', // string
    'by_ip_from_comment' => 'by_ip_from_comment_example', // string
    'filters' => 'filters_example', // string
    'search_filters' => 'search_filters_example', // string
    'sorts' => 'sorts_example', // string
    'demo' => True, // bool
    'sso' => 'sso_example', // string
];


try {
    $result = $apiInstance->getApiComments($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getApiComments: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]