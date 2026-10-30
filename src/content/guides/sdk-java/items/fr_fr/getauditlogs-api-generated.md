## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | Le locataire pour lequel lire les journaux d'audit. |
| limit | integer | query | No | Nombre de journaux à retourner. Max 10000, par défaut 1000. |
| skip | integer | query | No | Nombre de journaux à ignorer, pour la pagination. |
| order | string | query | No | Direction de tri sur le temps de l'événement. Par défaut DESC (le plus récent d'abord). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Retourner uniquement les événements effectués par ce nom d'utilisateur. |
| ip | string | query | No | Retourner uniquement les événements provenant de cette adresse IP. |
| crudType | string | query | No | Retourner uniquement les événements de ce type : c, r, u, d, ou connexion. |
| resourceName | string | query | No | Retourner uniquement les événements pour cette ressource, par ex. User ou Moderator. |
| targetId | string | query | No | Retourner uniquement les événements dont l'objet affecté possède cet identifiant. |
| target | string | query | No | Correspondance insensible à la casse sur le libellé de l'objet affecté, par ex. un nom d'utilisateur ou un e‑mail. |
| includeManagedTenants | boolean | query | No | Retourner également les événements des locataires gérés par ce locataire. |

## Réponse

Retourne : [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogsResponse.java)

## Exemple

[inline-code-attrs-start title = 'Exemple getAuditLogs'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importer les classes :
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.auth.*;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");
    
    // Configurer l'autorisation de la clé API : api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Décommentez la ligne suivante pour définir un préfixe pour la clé API, par ex. "Token" (valeur par défaut : null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | Le locataire pour lequel lire les journaux d'audit.
    Integer limit = 56; // Integer | Nombre de journaux à retourner. Max 10000, par défaut 1000.
    Integer skip = 56; // Integer | Nombre de journaux à ignorer, pour la pagination.
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | Direction de tri sur le temps de l'événement. Par défaut DESC (le plus récent d'abord).
    Long after = 56L; // Long | 
    Long before = 56L; // Long | 
    String username = "username_example"; // String | Retourner uniquement les événements effectués par ce nom d'utilisateur.
    String ip = "ip_example"; // String | Retourner uniquement les événements provenant de cette adresse IP.
    String crudType = "crudType_example"; // String | Retourner uniquement les événements de ce type : c, r, u, d, ou connexion.
    String resourceName = "resourceName_example"; // String | Retourner uniquement les événements pour cette ressource, par ex. User ou Moderator.
    String targetId = "targetId_example"; // String | Retourner uniquement les événements dont l'objet affecté possède cet identifiant.
    String target = "target_example"; // String | Correspondance insensible à la casse sur le libellé de l'objet affecté, par ex. un nom d'utilisateur ou un e‑mail.
    Boolean includeManagedTenants = true; // Boolean | Retourner également les événements des locataires gérés par ce locataire.
    try {
      GetAuditLogsResponse result = apiInstance.getAuditLogs(tenantId)
            .limit(limit)
            .skip(skip)
            .order(order)
            .after(after)
            .before(before)
            .username(username)
            .ip(ip)
            .crudType(crudType)
            .resourceName(resourceName)
            .targetId(targetId)
            .target(target)
            .includeManagedTenants(includeManagedTenants)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getAuditLogs");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]