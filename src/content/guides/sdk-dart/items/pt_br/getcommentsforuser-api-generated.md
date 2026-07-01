## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|------------|------------|-----------|
| userId | string | query | Não |  |
| direction | string | query | Não |  |
| repliesToUserId | string | query | Não |  |
| page | number | query | Não |  |
| includei10n | boolean | query | Não |  |
| locale | string | query | Não |  |
| isCrawler | boolean | query | Não |  |

## Resposta

Retorna: `GetCommentsForUserResponse`

## Exemplo

[inline-code-attrs-start title = 'Exemplo getCommentsForUser'; type = ''; isFunctional = false; inline-code-attrs-end]
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