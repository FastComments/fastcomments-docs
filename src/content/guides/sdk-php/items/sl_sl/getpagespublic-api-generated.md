Navede strani za najemnika. Uporablja namizni odjemalec FChat za polnjenje svojega seznama sob.
Zahteva, da je `enableFChat` nastavljen na true v razrešenih nastavitvah po meri za vsako stran.
Strani, ki zahtevajo SSO, so filtrirane glede na dostop do skupin zahtevajočega uporabnika.

## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| cursor | string | query | Ne | Neprozoren kazalec za paginacijo, vrnjen kot `nextCursor` iz prejšnje zahteve. Povezan z istim `sortBy`. |
| limit | integer | query | Ne | 1..200, privzeto 50 |
| q | string | query | Ne | Neobvezen filter predpone naslova, neobčutljiv na velikost črk. |
| sortBy | string | query | Ne | Vrstni red sortiranja. `updatedAt` (privzeto, najnovejše najprej), `commentCount` (največ komentarjev najprej) ali `title` (abecedno). |
| hasComments | boolean | query | Ne | Če je true, vrni samo strani z vsaj enim komentarjem. |

## Odgovor

Vrne: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Neprozoren kazalec za paginacijo, vrnjen kot `nextCursor` iz prejšnje zahteve. Povezan z istim `sortBy`.
    'limit' => 56, // int | 1..200, privzeto 50
    'q' => 'q_example', // string | Neobvezen filter predpone naslova, neobčutljiv na velikost črk.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Vrstni red sortiranja. `updatedAt` (privzeto, najnovejše najprej), `commentCount` (največ komentarjev najprej) ali `title` (abecedno).
    'has_comments' => True, // bool | Če je true, vrni samo strani z vsaj enim komentarjem.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]