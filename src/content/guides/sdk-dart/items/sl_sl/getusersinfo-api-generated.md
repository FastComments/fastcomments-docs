Informacije o množičnih uporabnikih za najemnika. Ob podanih userIds vrne prikazne informacije iz User / SSOUser.
Uporablja ga widget za komentarje za obogatitev uporabnikov, ki so se pravkar pojavili prek dogodka prisotnosti.
Brez konteksta strani: zasebnost se uveljavlja enotno (zasebni profili so maskirani).

## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| ids | string | query | Da | userIds, ločeni z vejico. |

## Odgovor

Vrne: `PageUsersInfoResponse`

## Primer

[inline-code-attrs-start title = 'Primer getUsersInfo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | userIds, ločeni z vejico.

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]