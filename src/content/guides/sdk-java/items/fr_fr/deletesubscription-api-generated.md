## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| id | string | path | Oui |  |
| userId | string | query | Non |  |

## Réponse

Retourne: [`DeleteSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/DeleteSubscriptionAPIResponse.java)

## Exemple

[inline-code-attrs-start title = 'Exemple de deleteSubscription'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importer les classes :
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
    
    // Configurer l'autorisation de clé API : api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Décommentez la ligne suivante pour définir un préfixe pour la clé API, par ex. "Token" (par défaut null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String id = "id_example"; // String | 
    String userId = "userId_example"; // String | 
    try {
      DeleteSubscriptionAPIResponse result = apiInstance.deleteSubscription(tenantId, id)
            .userId(userId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#deleteSubscription");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]