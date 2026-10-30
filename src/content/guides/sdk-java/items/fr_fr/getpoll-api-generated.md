## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| commentId | string | path | Oui |  |

## Réponse

Renvoie : [`GetPollResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPollResponse.java)

## Exemple

[inline-code-attrs-start title = 'Exemple getPoll'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importer les classes:
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
    // Décommentez la ligne suivante pour définir un préfixe pour la clé API, par ex. "Token" (valeur par défaut null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String commentId = "commentId_example"; // String | 
    try {
      GetPollResponse result = apiInstance.getPoll(tenantId, commentId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception lors de l'appel de DefaultApi#getPoll");
      System.err.println("Code d'état : " + e.getCode());
      System.err.println("Raison : " + e.getResponseBody());
      System.err.println("En-têtes de réponse : " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]