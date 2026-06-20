Popis stranica za tenant. Koristi se od strane FChat desktop klijenta za popunjavanje njegovog popisa soba.
Zahtijeva da `enableFChat` bude true u razriješenoj prilagođenoj konfiguraciji za svaku stranicu.
Stranice koje zahtijevaju SSO filtriraju se prema pristupnim grupama korisnika koji šalje zahtjev.

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Nečitljiv paginacijski kursor vraćen kao `nextCursor` iz prethodnog zahtjeva. Povezan s istim `sortBy`. |
| limit | integer | query | No | 1..200, zadano 50 |
| q | string | query | No | Neobavezni prefiks naslova koji ne razlikuje velika/mala slova. |
| sortBy | string | query | No | Redoslijed sortiranja. `updatedAt` (zadano, najnovije prvo), `commentCount` (stranice s najviše komentara prve), ili `title` (abecedno). |
| hasComments | boolean | query | No | Ako je true, vraćaju se samo stranice s barem jednim komentarom. |

## Odgovor

Vraća: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Primjer

[inline-code-attrs-start title = 'getPagesPublic Primjer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite koristiti prilagođeni HTTP klijent, proslijedite svoj klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opcionalno, kao zadani će se koristiti `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | Nečitljiv paginacijski kursor vraćen kao `nextCursor` iz prethodnog zahtjeva. Povezan s istim `sortBy`.
$limit = 56; // int | 1..200, zadano 50
$q = 'q_example'; // string | Neobavezni prefiks naslova koji ne razlikuje velika/mala slova.
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | Redoslijed sortiranja. `updatedAt` (zadano, najnovije prvo), `commentCount` (stranice s najviše komentara prve), ili `title` (abecedno).
$has_comments = True; // bool | Ako je true, vraćaju se samo stranice s barem jednim komentarom.

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]