---
Skupni podaci o korisnicima za najmodavca. Na temelju userIds, vraća informacije za prikaz iz User / SSOUser.  
Koristi se od strane widgeta za komentare kako bi obogatio korisnike koji su se upravo pojavili putem događaja prisutnosti.  
Nema konteksta stranice: privatnost se dosljedno provodi (privatni profili su maskirani).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | Korisnički ID-ovi odvojeni zarezom. |

## Response

Vraća: `PageUsersInfoResponse`

## Example

[inline-code-attrs-start title = 'Primjer getUsersInfo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | Korisnički ID-ovi odvojeni zarezom.

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]

---