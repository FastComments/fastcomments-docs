Massebrugerinfo for en lejer. Givet bruger‑id’er, returneres visningsinfo fra User / SSOUser.  
Brugt af kommentarmodulet til at berige brugere, der netop er dukket op via en tilstedeværelseshændelse.  
Ingen sidekontekst: privatliv håndhæves ensartet (private profiler maskeres).

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|-----------|----------|-------------|
| tenantId | string | path | Ja |  |
| ids | string | query | Ja | Komma‑adskilte bruger‑id’er. |

## Svar

Returnerer: `PageUsersInfoResponse`

## Eksempel

[inline-code-attrs-start title = 'getUsersInfo Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | Komma‑adskilte bruger‑id’er.

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]

---