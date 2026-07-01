## Parameter

| Name              | Typ   | Ort   | Erforderlich | Beschreibung |
|-------------------|-------|-------|--------------|--------------|
| tenantId          | string| query | Ja           |  |
| text-search       | string| query | Nein         |  |
| byIPFromComment   | string| query | Nein         |  |
| filter            | string| query | Nein         |  |
| searchFilters     | string| query | Nein         |  |
| demo              | boolean| query| Nein         |  |
| sso               | string| query | Nein         |  |

## Antwort

Rückgabe: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPICountCommentsResponse.php)

## Beispiel

[inline-code-attrs-start title = 'getCount Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'text_search' => 'text_search_example', // string
    'by_ip_from_comment' => 'by_ip_from_comment_example', // string
    'filter' => 'filter_example', // string
    'search_filters' => 'search_filters_example', // string
    'demo' => True, // bool
    'sso' => 'sso_example', // string
];


try {
    $result = $apiInstance->getCount($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]