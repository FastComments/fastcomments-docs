---
## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| page | number | query | Nie |  |
| count | number | query | Nie |  |
| text-search | string | query | Nie |  |
| byIPFromComment | string | query | Nie |  |
| filters | string | query | Nie |  |
| searchFilters | string | query | Nie |  |
| sorts | string | query | Nie |  |
| demo | boolean | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPIGetCommentsResponse.php)

## Przykład

[inline-code-attrs-start title = 'Przykład getApiComments'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Jeżeli chcesz użyć niestandardowego klienta HTTP, przekaż klienta, który implementuje `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne, domyślnie używany będzie `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$page = 3.4; // liczba zmiennoprzecinkowa (float)
$count = 3.4; // liczba zmiennoprzecinkowa (float)
$text_search = 'text_search_example'; // ciąg znaków (string)
$by_ip_from_comment = 'by_ip_from_comment_example'; // ciąg znaków (string)
$filters = 'filters_example'; // ciąg znaków (string)
$search_filters = 'search_filters_example'; // ciąg znaków (string)
$sorts = 'sorts_example'; // ciąg znaków (string)
$demo = True; // wartość logiczna (bool)
$sso = 'sso_example'; // ciąg znaków (string)

try {
    $result = $apiInstance->getApiComments($page, $count, $text_search, $by_ip_from_comment, $filters, $search_filters, $sorts, $demo, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getApiComments: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---