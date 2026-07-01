## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| userId | string | query | Non |  |
| state | number | query | Non |  |
| skip | number | query | Non |  |
| limit | number | query | Non |  |

## Réponse

Retourne : `GetTicketsResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple getTickets'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurer l’autorisation de clé API : api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Décommentez ci-dessous pour configurer le préfixe (par ex. Bearer) pour la clé API, si nécessaire
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final state = 1.2; // double | 
final skip = 1.2; // double | 
final limit = 1.2; // double | 

try {
    final result = api_instance.getTickets(tenantId, GetTicketsOptions(userId: userId, state: state, skip: skip, limit: limit));
    print(result);
} catch (e) {
    print('Exception lors de l\'appel de DefaultApi->getTickets: $e\n');
}
[inline-code-end]

---