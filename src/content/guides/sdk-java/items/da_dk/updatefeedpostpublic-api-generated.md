## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| postId | string | path | Ja |  |
| broadcastId | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`CreateFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateFeedPostPublic200Response.java)

## Eksempel

[inline-code-attrs-start title = 'updateFeedPostPublic Eksempel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importer klasser:
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
    UpdateFeedPostParams updateFeedPostParams = new UpdateFeedPostParams(); // UpdateFeedPostParams | 
    String broadcastId = "broadcastId_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      CreateFeedPostPublic200Response result = apiInstance.updateFeedPostPublic(tenantId, postId, updateFeedPostParams)
            .broadcastId(broadcastId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#updateFeedPostPublic");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]