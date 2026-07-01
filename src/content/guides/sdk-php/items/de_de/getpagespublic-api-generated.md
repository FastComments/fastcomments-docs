List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Undurchsichtiger Pagination‑Cursor, zurückgegeben als `nextCursor` von einer vorherigen Anfrage. Gebunden an denselben `sortBy`. |
| limit | integer | query | No | 1..200, Standard 50 |
| q | string | query | No | Optionaler, case‑insensitiver Titel‑Präfix‑Filter. |
| sortBy | string | query | No | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (die meisten Kommentare zuerst) oder `title` (alphabetisch). |
| hasComments | boolean | query | No | Wenn true, werden nur Seiten mit mindestens einem Kommentar zurückgegeben. |

## Antwort

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Beispiel

[inline-code-attrs-start title = 'getPagesPublic Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Wenn Sie einen benutzerdefinierten HTTP‑Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird als Standard verwendet.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | Undurchsichtiger Pagination‑Cursor, zurückgegeben als `nextCursor` von einer vorherigen Anfrage. Gebunden an denselben `sortBy`.
    'limit' => 56, // int | 1..200, Standard 50
    'q' => 'q_example', // string | Optionaler, case‑insensitiver Titel‑Präfix‑Filter.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (die meisten Kommentare zuerst) oder `title` (alphabetisch).
    'has_comments' => True, // bool | Wenn true, werden nur Seiten mit mindestens einem Kommentar zurückgegeben.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]