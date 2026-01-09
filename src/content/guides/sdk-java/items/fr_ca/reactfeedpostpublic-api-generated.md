## Paramètres

| Nom | Type | Location | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| postId | string | path | Oui |  |
| isUndo | boolean | query | Non |  |
| broadcastId | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/ReactFeedPostPublic200Response.java)

## Exemple

[inline-code-attrs-start title = 'Exemple reactFeedPostPublic'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importer les classes :
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.PublicApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    PublicApi apiInstance = new PublicApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String postId = "postId_example"; // String | 
    ReactBodyParams reactBodyParams = new ReactBodyParams(); // ReactBodyParams | 
    Boolean isUndo = true; // Boolean | 
    String broadcastId = "broadcastId_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      ReactFeedPostPublic200Response result = apiInstance.reactFeedPostPublic(tenantId, postId, reactBodyParams)
            .isUndo(isUndo)
            .broadcastId(broadcastId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#reactFeedPostPublic");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]