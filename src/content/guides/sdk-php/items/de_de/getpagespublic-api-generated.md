Listet Seiten für einen Tenant auf. Wird vom FChat-Desktop-Client verwendet, um seine Raumliste zu füllen.
Erfordert, dass `enableFChat` in der aufgelösten benutzerdefinierten Konfiguration für jede Seite auf true gesetzt ist.
Seiten, die SSO erfordern, werden gegen die Gruppenzugriffsrechte des anfragenden Benutzers gefiltert.

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| cursor | string | query | Nein | Opaker Paginierungs-Cursor, der als `nextCursor` von einer vorherigen Anfrage zurückgegeben wurde. An denselben `sortBy` gebunden. |
| limit | integer | query | Nein | 1..200, Standard 50 |
| q | string | query | Nein | Optionaler Filter für Titel-Präfix, Groß-/Kleinschreibung wird ignoriert. |
| sortBy | string | query | Nein | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (meiste Kommentare zuerst) oder `title` (alphabetisch). |
| hasComments | boolean | query | Nein | Wenn true, nur Seiten mit mindestens einem Kommentar zurückgeben. |

## Antwort

Gibt zurück: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## Beispiel

[inline-code-attrs-start title = 'getPagesPublic Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
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
    'cursor' => 'cursor_example', // string | Opaker Paginierungs-Cursor, der als `nextCursor` von einer vorherigen Anfrage zurückgegeben wurde. An denselben `sortBy` gebunden.
    'limit' => 56, // int | 1..200, Standard 50
    'q' => 'q_example', // string | Optionaler Filter für Titel-Präfix, Groß-/Kleinschreibung wird ignoriert.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (meiste Kommentare zuerst) oder `title` (alphabetisch).
    'has_comments' => True, // bool | Wenn true, nur Seiten mit mindestens einem Kommentar zurückgeben.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]