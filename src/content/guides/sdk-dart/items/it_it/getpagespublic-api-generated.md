Elenca le pagine per un tenant. Usato dal client desktop FChat per popolare la sua lista delle stanze.  
Richiede che `enableFChat` sia true nella configurazione personalizzata risolta per ogni pagina.  
Le pagine che richiedono SSO sono filtrate in base all'accesso al gruppo dell'utente richiedente.

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | path | Sì |  |
| cursor | string | query | No | Cursore di paginazione opaco restituito come `nextCursor` da una richiesta precedente. Legato allo stesso `sortBy`. |
| limit | integer | query | No | 1..200, default 50 |
| q | string | query | No | Filtro opzionale del prefisso del titolo, non sensibile a maiuscole/minuscole. |
| sortBy | string | query | No | Ordine di ordinamento. `updatedAt` (predefinito, più recente per prima), `commentCount` (più commenti per prima), o `title` (alfabetico). |
| hasComments | boolean | query | No | Se true, restituisce solo le pagine con almeno un commento. |

## Risposta

Restituisce: `GetPublicPagesResponse`

## Esempio

[inline-code-attrs-start title = 'Esempio getPagesPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Cursore di paginazione opaco restituito come `nextCursor` da una richiesta precedente. Legato allo stesso `sortBy`.
final limit = 56; // int | 1..200, default 50
final q = q_example; // String | Filtro opzionale del prefisso del titolo, non sensibile a maiuscole/minuscole.
final sortBy = ; // PagesSortBy | Ordine di ordinamento. `updatedAt` (predefinito, più recente per prima), `commentCount` (più commenti per prima), o `title` (alfabetico).
final hasComments = true; // bool | Se true, restituisce solo le pagine con almeno un commento.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]