## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tag | string | path | Da |  |
| tenantId | string | query | Ne |  |

## Odgovor

Vrne: [`PatchHashTag200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PatchHashTag200Response.java)

## Primer

[inline-code-attrs-start title = 'Primer patchHashTag'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Uvozi razrede:
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
    
    // Konfigurirajte avtorizacijo za API ključ: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Odkomentirajte naslednjo vrstico, da nastavite predpono za API ključ, npr. "Token" (privzeto null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tag = "tag_example"; // String | 
    String tenantId = "tenantId_example"; // String | 
    UpdateHashTagBody updateHashTagBody = new UpdateHashTagBody(); // UpdateHashTagBody | 
    try {
      PatchHashTag200Response result = apiInstance.patchHashTag(tag)
            .tenantId(tenantId)
            .updateHashTagBody(updateHashTagBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#patchHashTag");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]