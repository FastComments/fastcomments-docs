Nabraja stranice za najamnika (tenant). Koristi FChat desktop klijent za popunjavanje svog popisa soba.
Zahtijeva da `enableFChat` bude true na razriješenoj prilagođenoj konfiguraciji za svaku stranicu.
Stranice koje zahtijevaju SSO filtriraju se prema pristupu grupama korisnika koji šalje zahtjev.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| cursor | string | query | Ne | Neprozirni kursor paginacije vraćen kao `nextCursor` iz prethodnog zahtjeva. Povezan s istim `sortBy`. |
| limit | integer | query | Ne | 1..200, zadano 50 |
| q | string | query | Ne | Neobavezni filter prefiksa naslova, neosjetljiv na velika i mala slova. |
| sortBy | string | query | Ne | Redoslijed sortiranja. `updatedAt` (zadano, najnovije prvo), `commentCount` (najviše komentara prvo) ili `title` (abecedno). |
| hasComments | boolean | query | Ne | Ako je true, vraća samo stranice s barem jednim komentarom. |

## Odgovor

Vraća: `GetPublicPagesResponse`

## Primjer

[inline-code-attrs-start title = 'Primjer za getPagesPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Neprozirni kursor paginacije vraćen kao `nextCursor` iz prethodnog zahtjeva. Povezan s istim `sortBy`.
final limit = 56; // int | 1..200, zadano 50
final q = q_example; // String | Neobavezni filter prefiksa naslova, neosjetljiv na velika i mala slova.
final sortBy = ; // PagesSortBy | Redoslijed sortiranja. `updatedAt` (zadano, najnovije prvo), `commentCount` (najviše komentara prvo) ili `title` (abecedno).
final hasComments = true; // bool | Ako je true, vraća samo stranice s barem jednim komentarom.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]