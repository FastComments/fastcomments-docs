Seiten für einen Mandanten auflisten. Wird vom FChat-Desktop-Client verwendet, um seine Raumliste zu befüllen.
Erfordert, dass `enableFChat` in der aufgelösten benutzerdefinierten Konfiguration für jede Seite auf true gesetzt ist.
Seiten, die SSO erfordern, werden anhand des Gruppen-Zugriffs des anfragenden Benutzers gefiltert.

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Opaker Paginierungs-Cursor, der als `nextCursor` von einer vorherigen Anfrage zurückgegeben wurde. Ist an dasselbe `sortBy` gebunden. |
| limit | integer | query | No | 1..200, Standard 50 |
| q | string | query | No | Optionaler, groß-/kleinschreibungsunabhängiger Titel-Präfixfilter. |
| sortBy | string | query | No | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (die meisten Kommentare zuerst), oder `title` (alphabetisch). |
| hasComments | boolean | query | No | Wenn true, nur Seiten mit mindestens einem Kommentar zurückgeben. |

## Antwort

Gibt zurück: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Beispiel

[inline-code-attrs-start title = 'getPagesPublic Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | Opaker Paginierungs-Cursor, der als `nextCursor` von einer vorherigen Anfrage zurückgegeben wurde. An dasselbe `sortBy` gebunden.
$limit = 56; // int | 1..200, Standard 50
$q = 'q_example'; // string | Optionaler, groß-/kleinschreibungsunabhängiger Titel-Präfixfilter.
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (die meisten Kommentare zuerst) oder `title` (alphabetisch).
$has_comments = True; // bool | Wenn true, nur Seiten mit mindestens einem Kommentar zurückgeben.

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]