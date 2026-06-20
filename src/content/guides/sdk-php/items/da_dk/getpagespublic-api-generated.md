Lister sider for en tenant. Bruges af FChat desktop-klienten til at udfylde sin rumliste.
Kræver, at `enableFChat` er true i den endelige brugerdefinerede konfiguration for hver side.
Sider, der kræver SSO, filtreres ud fra den anmodende brugers gruppeadgang.

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Opaque pagineringscursor returneret som `nextCursor` fra en tidligere anmodning. Bundet til samme `sortBy`. |
| limit | integer | query | No | 1..200, standard 50 |
| q | string | query | No | Valgfrit titel-præfiksfilter uden hensyn til store/små bogstaver. |
| sortBy | string | query | No | Sorteringsrækkefølge. `updatedAt` (standard, nyeste først), `commentCount` (flest kommentarer først), eller `title` (alfabetisk). |
| hasComments | boolean | query | No | Hvis true, returner kun sider med mindst én kommentar. |

## Response

Returnerer: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Eksempel

[inline-code-attrs-start title = 'getPagesPublic Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Hvis du ønsker at bruge en brugerdefineret HTTP-klient, skal du videregive din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit; `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | Opaque pagineringscursor returneret som `nextCursor` fra en tidligere forespørgsel. Tilknyttet samme `sortBy`.
$limit = 56; // int | 1..200, standard 50
$q = 'q_example'; // string | Valgfrit titelpræfiksfilter uden hensyn til store/små bogstaver.
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | Sorteringsrækkefølge. `updatedAt` (standard, nyeste først), `commentCount` (flest kommentarer først), eller `title` (alfabetisk).
$has_comments = True; // bool | Hvis true, returnér kun sider med mindst én kommentar.

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]