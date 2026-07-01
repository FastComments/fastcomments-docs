Masovne informacije o korisnicima za zakupca. Na osnovu userId‑ova, vraća se prikazna informacija iz User / SSOUser. Koristi se u vidžetu za komentare kako bi obogatio korisnike koji su upravo pojavljeni putem događaja prisutnosti. Bez konteksta stranice: privatnost se dosledno primenjuje (privatni profili su maskirani).

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | UserId‑ovi odvojeni zarezom. |

## Odgovor

Vraća: `PageUsersInfoResponse`

## Primer

[inline-code-attrs-start title = 'Primer getUsersInfo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | UserId‑ovi odvojeni zarezom.

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]