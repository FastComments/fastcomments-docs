List sider for en lejer. Bruges af FChat desktop‑klienten til at udfylde dens rumliste.  
Kræver, at `enableFChat` er sand på den løste brugerdefinerede konfiguration for hver side.  
Sider, der kræver SSO, filtreres i forhold til den anmodende brugers gruppeadgang.

## Parameters

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|-----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Uigennemsigtigt pagineringscursor returneret som `nextCursor` fra en tidligere anmodning. Knyttet til den samme `sortBy`. |
| limit | integer | query | No | 1..200, standard 50 |
| q | string | query | No | Valgfri case‑insensitiv titelpræfiksfilter. |
| sortBy | string | query | No | Sorteringsrækkefølge. `updatedAt` (standard, nyeste først), `commentCount` (fleste kommentarer først), eller `title` (alfabetisk). |
| hasComments | boolean | query | No | Hvis sand, returnerer kun sider med mindst én kommentar. |

## Response

Returnerer: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Example

[inline-code-attrs-start title = 'getPagesPublic Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Hvis du vil bruge en brugerdefineret http-klient, skal du videregive din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Uigennemsigtigt pagineringscursor returneret som `nextCursor` fra en tidligere anmodning. Knyttet til den samme `sortBy`.
    'limit' => 56, // int | 1..200, standard 50
    'q' => 'q_example', // string | Valgfri case‑insensitiv titelpræfiksfilter.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Sorteringsrækkefølge. `updatedAt` (standard, nyeste først), `commentCount` (fleste kommentarer først), eller `title` (alfabetisk).
    'has_comments' => True, // bool | Hvis sand, returnerer kun sider med mindst én kommentar.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---