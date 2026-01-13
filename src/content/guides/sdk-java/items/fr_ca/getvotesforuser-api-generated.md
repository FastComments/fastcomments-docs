## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| urlId | string | query | Oui |  |
| userId | string | query | Non |  |
| anonUserId | string | query | Non |  |

## Réponse

Retourne: [`GetVotesForUser200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetVotesForUser200Response.java)

## Exemple

[inline-code-attrs-start title = 'Exemple getVotesForUser'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // Configurer l'authentification par clé API : api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Décommentez la ligne suivante pour définir un préfixe pour la clé API, p.ex. "Token" (par défaut null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String urlId = "urlId_example"; // String | 
    String userId = "userId_example"; // String | 
    String anonUserId = "anonUserId_example"; // String | 
    try {
      GetVotesForUser200Response result = apiInstance.getVotesForUser(tenantId, urlId)
            .userId(userId)
            .anonUserId(anonUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getVotesForUser");
      // Exception lors de l'appel de DefaultApi#getVotesForUser
      System.err.println("Status code: " + e.getCode());
      // Code d'état : 
      System.err.println("Reason: " + e.getResponseBody());
      // Raison : 
      System.err.println("Response headers: " + e.getResponseHeaders());
      // En-têtes de réponse : 
      e.printStackTrace();
    }
  }
}
[inline-code-end]