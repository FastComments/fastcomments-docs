Lijst pagina's voor een tenant. Gebruikt door de FChat-desktopclient om zijn kamerlijst te vullen. Vereist dat `enableFChat` op true staat in de opgeloste aangepaste configuratie voor elke pagina. Pagina's die SSO vereisen worden gefilterd op basis van de groepsrechten van de aanvragende gebruiker.

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Ondoorzichtige pagineringscursor teruggegeven als `nextCursor` van een eerdere aanvraag. Gekoppeld aan dezelfde `sortBy`. |
| limit | integer | query | No | 1..200, standaard 50 |
| q | string | query | No | Optionele niet-hoofdlettergevoelige titel-prefixfilter. |
| sortBy | string | query | No | Sorteervolgorde. `updatedAt` (standaard, meest recent eerst), `commentCount` (meeste reacties eerst), of `title` (alfabetisch). |
| hasComments | boolean | query | No | Als true, geef alleen pagina's terug met ten minste één reactie. |

## Response

Geeft terug: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Voorbeeld

[inline-code-attrs-start title = 'getPagesPublic Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Als u een custom http-client wilt gebruiken, geef dan uw client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel; `GuzzleHttp\Client` wordt standaard gebruikt.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | Ondoorzichtige pagineringscursor teruggegeven als `nextCursor` van een eerdere aanvraag. Gekoppeld aan dezelfde `sortBy`.
$limit = 56; // int | 1..200, standaard 50
$q = 'q_example'; // string | Optionele niet-hoofdlettergevoelige titel-prefixfilter.
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | Sorteervolgorde. `updatedAt` (standaard, meest recent eerst), `commentCount` (meeste reacties eerst), of `title` (alfabetisch).
$has_comments = True; // bool | Als true, geef alleen pagina's terug met ten minste één reactie.

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]