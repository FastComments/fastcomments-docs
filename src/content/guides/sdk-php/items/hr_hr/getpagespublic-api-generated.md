List pages for a tenant. Used by the FChat desktop client to populate its room list.
Requires `enableFChat` to be true on the resolved custom config for each page.
Pages that require SSO are filtered against the requesting user's group access.

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| cursor | string | query | Ne | Neprozirni kursorski pokazivač za paginaciju vraćen kao `nextCursor` iz prethodnog zahtjeva. Povezano s istim `sortBy`. |
| limit | integer | query | Ne | 1..200, zadano 50 |
| q | string | query | Ne | Opcionalni filter prefiksa naslova koji ne razlikuje veličinu slova. |
| sortBy | string | query | Ne | Redoslijed sortiranja. `updatedAt` (zadano, najnoviji prvi), `commentCount` (najviše komentara prvi), ili `title` (abecedno). |
| hasComments | boolean | query | Ne | Ako je true, vrati samo stranice s barem jednim komentarom. |

## Odgovor

Vraća: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Primjer

[inline-code-attrs-start title = 'getPagesPublic Primjer'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite koristiti prilagođeni HTTP klijent, proslijedite svoj klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opcionalno, `GuzzleHttp\Client` će se koristiti kao zadano.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Neprozirni kursorski pokazivač za paginaciju vraćen kao `nextCursor` iz prethodnog zahtjeva. Povezano s istim `sortBy`.
    'limit' => 56, // int | 1..200, zadano 50
    'q' => 'q_example', // string | Opcionalni filter prefiksa naslova koji ne razlikuje veličinu slova.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Redoslijed sortiranja. `updatedAt` (zadano, najnoviji prvi), `commentCount` (najviše komentara prvi), ili `title` (abecedno).
    'has_comments' => True, // bool | Ako je true, vrati samo stranice s barem jednim komentarom.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---