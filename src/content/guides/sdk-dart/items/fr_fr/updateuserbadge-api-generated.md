## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| id | string | path | Oui |  |

## Réponse

Returns: `APIEmptySuccessResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple de mise à jour du badge d\'utilisateur'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurer l'autorisation de la clé API : api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// décommenter ci-dessous pour configurer le préfixe (par ex. Bearer) pour la clé API, si nécessaire
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String |
final id = id_example; // String |
final updateUserBadgeParams = UpdateUserBadgeParams(); // UpdateUserBadgeParams | 

try {
    final result = api_instance.updateUserBadge(tenantId, id, updateUserBadgeParams);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateUserBadge: $e\n');
}
[inline-code-end]

---