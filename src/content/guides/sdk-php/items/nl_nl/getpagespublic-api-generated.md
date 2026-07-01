List pagina's voor een tenant. Gebruikt door de FChat desktopclient om zijn kamerlijst te vullen.  
Vereist dat `enableFChat` true is in de opgeloste aangepaste configuratie voor elke pagina.  
Paginas die SSO vereisen worden gefilterd op basis van de groepstoegang van de vraagende gebruiker.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Opaque paginatiecursor geretourneerd als `nextCursor` van een eerdere aanvraag. Gebonden aan dezelfde `sortBy`. |
| limit | integer | query | No | 1..200, standaard 50 |
| q | string | query | No | Optionele hoofdletterongevoelige titelvoorvoegselfilter. |
| sortBy | string | query | No | Sorteervolgorde. `updatedAt` (standaard, nieuwste eerst), `commentCount` (meeste reacties eerst), of `title` (alfabetisch). |
| hasComments | boolean | query | No | Als true, alleen pagina's retourneren met ten minste één reactie. |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Example

[inline-code-attrs-start title = 'getPagesPublic Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Als u een aangepaste HTTP-client wilt gebruiken, geef dan uw client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel, `GuzzleHttp\Client` wordt standaard gebruikt.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Opaque paginatiecursor geretourneerd als `nextCursor` van een eerdere aanvraag. Gebonden aan dezelfde `sortBy`.
    'limit' => 56, // int | 1..200, standaard 50
    'q' => 'q_example', // string | Optionele hoofdletterongevoelige titelvoorvoegselfilter.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Sorteervolgorde. `updatedAt` (standaard, nieuwste eerst), `commentCount` (meeste reacties eerst), of `title` (alfabetisch).
    'has_comments' => True, // bool | Als true, alleen pagina's retourneren met ten minste één reactie.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]