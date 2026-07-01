List pages for a tenant. Used by the FChat desktop client to populate its room list.
Requires `enableFChat` to be true on the resolved custom config for each page.
Pages that require SSO are filtered against the requesting user's group access.

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Undurchsichtiger Paginierungs‑Cursor, zurückgegeben als `nextCursor` von einer vorherigen Anforderung. Gebunden an das gleiche `sortBy`. |
| limit | integer | query | No | 1..200, Standard 50 |
| q | string | query | No | Optionaler, nicht‑groß-/kleinschreibungsempfindlicher Titelfilter‑Präfix. |
| sortBy | string | query | No | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (meiste Kommentare zuerst) oder `title` (alphabetisch). |
| hasComments | boolean | query | No | Wenn true, nur Seiten zurückgeben, die mindestens einen Kommentar enthalten. |

## Antwort

Rückgabe: `GetPublicPagesResponse`

## Beispiel

[inline-code-attrs-start title = 'Beispiel getPagesPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String |
final cursor = cursor_example; // String | Undurchsichtiger Paginierungs‑Cursor, zurückgegeben als `nextCursor` von einer vorherigen Anforderung. Gebunden an das gleiche `sortBy`.
final limit = 56; // int | 1..200, Standard 50
final q = q_example; // String | Optionaler, nicht‑groß-/kleinschreibungsempfindlicher Titelfilter‑Präfix.
final sortBy = ; // PagesSortBy | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (meiste Kommentare zuerst) oder `title` (alphabetisch).
final hasComments = true; // bool | Wenn true, nur Seiten zurückgeben, die mindestens einen Kommentar enthalten.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]