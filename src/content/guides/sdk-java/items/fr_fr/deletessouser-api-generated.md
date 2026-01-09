## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| id | string | path | Oui |  |
| deleteComments | boolean | query | Non |  |
| commentDeleteMode | string | query | Non |  |

## Réponse

Retourne : [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/DeleteSSOUserAPIResponse.java)

## Exemple

[inline-code-attrs-start title = 'Exemple de deleteSSOUser'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String id = "id_example"; // String | 
    Boolean deleteComments = true; // Boolean | 
    String commentDeleteMode = "commentDeleteMode_example"; // String | 
    try {
      DeleteSSOUserAPIResponse result = apiInstance.deleteSSOUser(tenantId, id)
            .deleteComments(deleteComments)
            .commentDeleteMode(commentDeleteMode)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#deleteSSOUser");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]