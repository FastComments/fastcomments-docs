List pages for a tenant. Used by the FChat desktop client to populate its room list.
Requires `enableFChat` to be true on the resolved custom config for each page.
Pages that require SSO are filtered against the requesting user's group access.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Neprozirni paginacijski kursor vraćen kao `nextCursor` iz prethodnog zahtjeva. Veza na isti `sortBy`. |
| limit | integer | query | No | 1..200, podrazumevano 50 |
| q | string | query | No | Opcionalni filter naslova koji nije osjetljiv na veličinu slova, prema prefiksu. |
| sortBy | string | query | No | Redoslijed sortiranja. `updatedAt` (podrazumevano, najnovije prvo), `commentCount` (najviše komentara prvo), ili `title` (alfabetički). |
| hasComments | boolean | query | No | Ako je true, vraća samo stranice koje imaju najmanje jedan komentar. |

## Odgovor

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Primer

[inline-code-attrs-start title = 'Primer getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite koristiti prilagođeni http klijent, proslijedite vaš klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opcionalno, `GuzzleHttp\Client` će se koristiti podrazumevano.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Neprozirni paginacijski kursor vraćen kao `nextCursor` iz prethodnog zahtjeva. Veza na isti `sortBy`.
    'limit' => 56, // int | 1..200, podrazumevano 50
    'q' => 'q_example', // string | Opcionalni filter naslova koji nije osjetljiv na veličinu slova, prema prefiksu.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Redoslijed sortiranja. `updatedAt` (podrazumevano, najnovije prvo), `commentCount` (najviše komentara prvo), ili `title` (alfabetički).
    'has_comments' => True, // bool | Ako je true, vraća samo stranice koje imaju najmanje jedan komentar.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]