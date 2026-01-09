req
tenantId
afterId

## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| afterId | string | query | Non |  |
| limit | integer | query | Non |  |
| tags | array | query | Non |  |

## Réponse

Retourne: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetFeedPosts200Response.java)

## Exemple

[inline-code-attrs-start title = 'Exemple de getFeedPosts'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // Configurer l'authentification par clé API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Décommentez la ligne suivante pour définir un préfixe pour la clé API, p.ex. "Token" (par défaut null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String afterId = "afterId_example"; // String | 
    Integer limit = 56; // Integer | 
    List<String> tags = Arrays.asList(); // List<String> | 
    try {
      GetFeedPosts200Response result = apiInstance.getFeedPosts(tenantId)
            .afterId(afterId)
            .limit(limit)
            .tags(tags)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getFeedPosts");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]