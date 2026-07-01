Bulkgebruiksinformatie voor een tenant. Gegeven userIds, retourneert we weergave‑informatie van User / SSOUser. Gebruikt door de commentaarwidget om gebruikers die net verschenen via een presence‑event te verrijken. Geen paginacontext: privacy wordt uniform afgedwongen (privéprofielen worden gemaskeerd).

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|--------------|
| tenantId | string | pad | Ja |  |
| ids | string | query | Ja | Komma‑gescheiden userIds. |

## Response

Returns: `PageUsersInfoResponse`

## Example

[inline-code-attrs-start title = 'getUsersInfo Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | Komma-gescheiden userIds.

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]