Navodi stranice za zakupca (tenant). Koristi FChat desktop klijent za popunjavanje liste soba.
Zahteva da `enableFChat` bude true na razrešenoj prilagođenoj konfiguraciji za svaku stranicu.
Stranice koje zahtevaju SSO se filtriraju prema pristupu grupi tražioca.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| cursor | string | query | Ne | Neprozirni kursor paginacije vraćen kao `nextCursor` iz prethodnog zahteva. Vezan za isti `sortBy`. |
| limit | integer | query | Ne | 1..200, podrazumevano 50 |
| q | string | query | Ne | Opcioni filter prefiksa naslova, neosetljiv na velika i mala slova. |
| sortBy | string | query | Ne | Redosled sortiranja. `updatedAt` (podrazumevano, najnovije prvo), `commentCount` (najviše komentara prvo) ili `title` (alfabetski). |
| hasComments | boolean | query | Ne | Ako je true, vraća samo stranice sa bar jednim komentarom. |

## Odgovor

Vraća: `GetPublicPagesResponse`

## Primer

[inline-code-attrs-start title = 'getPagesPublic Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Neprozirni kursor paginacije vraćen kao `nextCursor` iz prethodnog zahteva. Vezan za isti `sortBy`.
final limit = 56; // int | 1..200, podrazumevano 50
final q = q_example; // String | Opcioni filter prefiksa naslova, neosetljiv na velika i mala slova.
final sortBy = ; // PagesSortBy | Redosled sortiranja. `updatedAt` (podrazumevano, najnovije prvo), `commentCount` (najviše komentara prvo) ili `title` (alfabetski).
final hasComments = true; // bool | Ako je true, vraća samo stranice sa bar jednim komentarom.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]