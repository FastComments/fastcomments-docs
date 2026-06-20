## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationSuggestResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer getSearchSuggest'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Če želite uporabiti prilagojen HTTP odjemalec, posredujte odjemalca, ki implementira `GuzzleHttp\ClientInterface`.
    // To je neobvezno, privzeto bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$text_search = 'text_search_example'; // niz
$sso = 'sso_example'; // niz

try {
    $result = $apiInstance->getSearchSuggest($text_search, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getSearchSuggest: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]