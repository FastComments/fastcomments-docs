Lista stranica za zakupca. Koristi FChat desktop klijent da popuni listu soba.  
Zahteva da `enableFChat` bude true u razrešenoj prilagođenoj konfiguraciji za svaku stranicu.  
Stranice koje zahtevaju SSO se filtriraju prema grupnom pristupu traženog korisnika.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Opaque pagination cursor vraćen kao `nextCursor` iz prethodnog zahteva. Povezan je sa istim `sortBy`. |
| limit | integer | query | No | 1..200, podrazumevano 50 |
| q | string | query | No | Opcioni filter prefiksa naslova koji ne pravi razliku između velikih i malih slova. |
| sortBy | string | query | No | Redosled sortiranja. `updatedAt` (podrazumevano, najnovije prvo), `commentCount` (najviše komentara prvo) ili `title` (abecedno). |
| hasComments | boolean | query | No | Ako je true, vraća se samo stranice koje imaju bar jedan komentar. |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Example

[inline-code-attrs-start title = 'Primer getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite koristiti prilagođeni HTTP klijent, prosledite svoj klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opcionalno, `GuzzleHttp\Client` će se koristiti podrazumevano.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Opaque pagination cursor vraćen kao `nextCursor` iz prethodnog zahteva. Povezan je sa istim `sortBy`.
    'limit' => 56, // int | 1..200, podrazumevano 50
    'q' => 'q_example', // string | Opcioni filter prefiksa naslova koji ne pravi razliku između velikih i malih slova.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Redosled sortiranja. `updatedAt` (podrazumevano, najnovije prvo), `commentCount` (najviše komentara prvo) ili `title` (abecedno).
    'has_comments' => True, // bool | Ako je true, vraća se samo stranice koje imaju bar jedan komentar.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]