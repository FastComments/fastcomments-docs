Prikaže strani za najemnika. Uporablja ga namizni odjemalec FChat za napolnitev svojega seznama sob.
Zahteva, da je `enableFChat` nastavljeno na true v razrešeni prilagojeni konfiguraciji za vsako stran.
Strani, ki zahtevajo SSO, so filtrirane glede na skupinski dostop uporabnika, ki pošilja zahtevo.

## Parametri

| Ime | Tip | Lokacija | Zahtevano | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| cursor | string | query | Ne | Neprozoren kazalec za straničenje, vrnjen kot `nextCursor` iz prejšnje zahteve. Povezan z istim `sortBy`. |
| limit | integer | query | Ne | 1..200, privzeto 50 |
| q | string | query | Ne | Izbirni filter predpon naslova, neobčutljiv na velike/male črke. |
| sortBy | string | query | Ne | Vrstni red sortiranja. `updatedAt` (privzeto, najnovejše prvi), `commentCount` (strani z največ komentarji prvi), ali `title` (po abecedi). |
| hasComments | boolean | query | Ne | Če je true, vrni samo strani z vsaj enim komentarjem. |

## Odgovor

Vrne: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Če želite uporabiti prilagojen HTTP odjemalec, posredujte odjemalca, ki implementira `GuzzleHttp\ClientInterface`.
    // To je opcijsko, kot privzet bo uporabljen `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | Neprozoren kazalec za straničenje, vrnjen kot `nextCursor` iz prejšnje zahteve. Povezan z istim `sortBy`.
$limit = 56; // int | 1..200, privzeto 50
$q = 'q_example'; // string | Izbirni filter predpon naslova, neobčutljiv na velike/male črke.
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | Vrstni red sortiranja. `updatedAt` (privzeto, najnovejše prvi), `commentCount` (strani z največ komentarji prvi), ali `title` (po abecedi).
$has_comments = True; // bool | Če je true, vrni samo strani z vsaj enim komentarjem.

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]