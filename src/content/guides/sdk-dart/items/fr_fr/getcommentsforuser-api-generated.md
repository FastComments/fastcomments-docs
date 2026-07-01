## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Non |  |
| direction | string | query | Non |  |
| repliesToUserId | string | query | Non |  |
| page | number | query | Non |  |
| includei10n | boolean | query | Non |  |
| locale | string | query | Non |  |
| isCrawler | boolean | query | Non |  |

## Réponse

Retourne : `GetCommentsForUserResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple getCommentsForUser'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final userId = userId_example; // String | 
final direction = ; // SortDirections | 
final repliesToUserId = repliesToUserId_example; // String | 
final page = 1.2; // double | 
final includei10n = true; // bool | 
final locale = locale_example; // String | 
final isCrawler = true; // bool | 

try {
    final result = api_instance.getCommentsForUser(GetCommentsForUserOptions(userId: userId, direction: direction, repliesToUserId: repliesToUserId, page: page, includei10n: includei10n, locale: locale, isCrawler: isCrawler));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getCommentsForUser: $e\n');
}
[inline-code-end]