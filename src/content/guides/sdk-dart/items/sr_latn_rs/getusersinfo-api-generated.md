Masovni podaci o korisnicima za tenant. Dati **userIds**, vraća prikazne informacije iz **User / SSOUser**.  
Koristi se od strane vidžeta za komentare da obogati korisnike koji su se upravo pojavili putem **presence** događaja.  
Nema konteksta stranice: privatnost se primenjuje uniformno (privatni profili su maskirani).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| ids | string | query | Da | UserId‑e odvojeni zarezom. |

## Response

Returns: `PageUsersInfoResponse`

## Primer

[inline-code-attrs-start title = 'getUsersInfo Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | UserId‑e odvojeni zarezom.

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]