Listet Seiten für einen Tenant auf. Wird vom FChat-Desktop-Client verwendet, um seine Raumliste zu füllen.
Erfordert, dass `enableFChat` in der aufgelösten benutzerdefinierten Konfiguration für jede Seite auf true steht.
Seiten, die SSO erfordern, werden gegen die Gruppenzugriffsrechte des anfragenden Benutzers gefiltert.

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| cursor | string | query | Nein | Opaker Paginierungs-Cursor, der als `nextCursor` von einer vorherigen Anfrage zurückgegeben wurde. An denselben `sortBy` gebunden. |
| limit | integer | query | Nein | 1..200, Standard 50 |
| q | string | query | Nein | Optionaler Filter für Titel-Präfix, Groß-/Kleinschreibung wird nicht beachtet. |
| sortBy | string | query | Nein | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (meiste Kommentare zuerst) oder `title` (alphabetisch). |
| hasComments | boolean | query | Nein | Wenn true, werden nur Seiten mit mindestens einem Kommentar zurückgegeben. |

## Antwort

Rückgabe: `GetPublicPagesResponse`

## Beispiel

[inline-code-attrs-start title = 'getPagesPublic Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Opaker Paginierungs-Cursor, der als `nextCursor` von einer vorherigen Anfrage zurückgegeben wurde. An denselben `sortBy` gebunden.
final limit = 56; // int | 1..200, Standard 50
final q = q_example; // String | Optionaler Filter für Titel-Präfix, Groß-/Kleinschreibung wird nicht beachtet.
final sortBy = ; // PagesSortBy | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (meiste Kommentare zuerst) oder `title` (alphabetisch).
final hasComments = true; // bool | Wenn true, werden nur Seiten mit mindestens einem Kommentar zurückgegeben.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]