List pages for a tenant. Used by the FChat desktop client to populate its room list.
Requires `enableFChat` to be true on the resolved custom config for each page.
Pages that require SSO are filtered against the requesting user's group access.

## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Neprosojen kurzor za paginacijo, vrnjen kot `nextCursor` iz prejšnje zahteve. Povezan z istim `sortBy`. |
| limit | integer | query | No | 1..200, privzeto 50 |
| q | string | query | No | Neobvezni filter predpone naslova, neobčutljiv na velikost črk. |
| sortBy | string | query | No | Vrstni red. `updatedAt` (privzeto, najnovejši najprej), `commentCount` (največ komentarjev najprej) ali `title` (abecedno). |
| hasComments | boolean | query | No | Če je true, vrne samo strani z vsaj enim komentarjem. |

## Odgovor

Vrne: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Primer

[inline-code-attrs-start title = 'getPagesPublic Primer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Če želite uporabiti prilagojeni HTTP odjemalec, podajte svoj odjemalec, ki implementira `GuzzleHttp\ClientInterface`.
    // To je neobvezno, kot privzeto bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Neprosojen kurzor za paginacijo, vrnjen kot `nextCursor` iz prejšnje zahteve. Povezan z istim `sortBy`.
    'limit' => 56, // int | 1..200, privzeto 50
    'q' => 'q_example', // string | Neobvezni filter predpone naslova, neobčutljiv na velikost črk.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Vrstni red. `updatedAt` (privzeto, najnovejši najprej), `commentCount` (največ komentarjev najprej) ali `title` (abecedno).
    'has_comments' => True, // bool | Če je true, vrne samo strani z vsaj enim komentarjem.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---