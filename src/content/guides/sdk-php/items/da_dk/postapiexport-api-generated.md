## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Nej |  |
| byIPFromComment | string | query | Nej |  |
| filters | string | query | Nej |  |
| searchFilters | string | query | Nej |  |
| sorts | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationExportResponse.php)

## Eksempel

[inline-code-attrs-start title = 'postApiExport Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, skal du angive din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client()
);
$text_search = 'text_search_example'; // string
$by_ip_from_comment = 'by_ip_from_comment_example'; // string
$filters = 'filters_example'; // string
$search_filters = 'search_filters_example'; // string
$sorts = 'sorts_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postApiExport($text_search, $by_ip_from_comment, $filters, $search_filters, $sorts, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postApiExport: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]