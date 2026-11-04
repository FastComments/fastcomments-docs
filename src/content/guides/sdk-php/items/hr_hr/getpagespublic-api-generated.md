Nabroji stranice za najmoprimca (tenant). Koristi FChat desktop klijent za popunjavanje svog popisa soba.
Zahtijeva da `enableFChat` bude true na razriješenoj prilagođenoj konfiguraciji za svaku stranicu.
Stranice koje zahtijevaju SSO filtriraju se prema pristupu grupi korisnika koji šalje zahtjev.

## Parametri

| Naziv | Vrsta | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| cursor | string | query | Ne | Neprozirni kursor paginacije vraćen kao `nextCursor` iz prethodnog zahtjeva. Vezan uz isti `sortBy`. |
| limit | integer | query | Ne | 1..200, zadano 50 |
| q | string | query | Ne | Neobavezi filter prefiksa naslova, neosjetljiv na velika i mala slova. |
| sortBy | string | query | Ne | Redoslijed sortiranja. `updatedAt` (zadano, najnovije prvo), `commentCount` (najviše komentara prvo) ili `title` (abecedno). |
| hasComments | boolean | query | Ne | Ako je true, vraća samo stranice s barem jednim komentarom. |

## Odgovor

Vraća: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Primjer

[inline-code-attrs-start title = 'getPagesPublic primjer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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
    'cursor' => 'cursor_example', // string | Neprozirni kursor paginacije vraćen kao `nextCursor` iz prethodnog zahtjeva. Vezan uz isti `sortBy`.
    'limit' => 56, // int | 1..200, zadano 50
    'q' => 'q_example', // string | Neobavezi filter prefiksa naslova, neosjetljiv na velika i mala slova.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Redoslijed sortiranja. `updatedAt` (zadano, najnovije prvo), `commentCount` (najviše komentara prvo) ili `title` (abecedno).
    'has_comments' => True, // bool | Ako je true, vraća samo stranice s barem jednim komentarom.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]