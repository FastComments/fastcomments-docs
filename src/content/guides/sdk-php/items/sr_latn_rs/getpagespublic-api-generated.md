---
Lista stranica za tenant. Koristi se od strane FChat desktop klijenta da popuni svoj spisak soba.
Zahteva da `enableFChat` bude true u razrešenoj prilagođenoj konfiguraciji za svaku stranicu.
Stranice koje zahtevaju SSO filtriraju se u skladu sa pristupom grupa korisnika koji podnosi zahtev.

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Neprozirni paginacioni kursor vraćen kao `nextCursor` iz prethodnog zahteva. Veže se za isti `sortBy`. |
| limit | integer | query | No | 1..200, podrazumevano 50 |
| q | string | query | No | Opcioni filter po prefiksu naslova, neosetljiv na velika/mala slova. |
| sortBy | string | query | No | Redosled sortiranja. `updatedAt` (podrazumevano, najnovije prve), `commentCount` (prvo najviše komentara), ili `title` (abecedno). |
| hasComments | boolean | query | No | Ako je true, vraćati samo stranice koje imaju bar jedan komentar. |

## Odgovor

Vraća: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite da koristite prilagođeni http klijent, prosledite klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opciono, `GuzzleHttp\Client` će biti korišćen kao podrazumevani.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | Neprozirni paginacioni kursor vraćen kao `nextCursor` iz prethodnog zahteva. Veže se za isti `sortBy`.
$limit = 56; // int | 1..200, podrazumevano 50
$q = 'q_example'; // string | Opcioni filter po prefiksu naslova koji nije osetljiv na velika/mala slova.
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | Redosled sortiranja. `updatedAt` (podrazumevano, najnovije prve), `commentCount` (prvo najviše komentara), ili `title` (abecedno).
$has_comments = True; // bool | Ako je true, vraćati samo stranice koje imaju bar jedan komentar.

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---