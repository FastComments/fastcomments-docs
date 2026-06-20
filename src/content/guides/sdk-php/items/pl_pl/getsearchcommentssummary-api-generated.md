## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| value | string | query | Nie |  |
| filters | string | query | Nie |  |
| searchFilters | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationCommentSearchResponse.php)

## Przykład

[inline-code-attrs-start title = 'Przykład getSearchCommentsSummary'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Jeśli chcesz użyć niestandardowego klienta HTTP, przekaż klienta, który implementuje `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne, domyślnie zostanie użyty `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$value = 'value_example'; // string
$filters = 'filters_example'; // string
$search_filters = 'search_filters_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getSearchCommentsSummary($value, $filters, $search_filters, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getSearchCommentsSummary: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]