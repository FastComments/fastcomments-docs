## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| page | integer | query | Ne |  |
| limit | integer | query | Ne |  |
| skip | integer | query | Ne |  |
| asTree | boolean | query | Ne |  |
| skipChildren | integer | query | Ne |  |
| limitChildren | integer | query | Ne |  |
| maxTreeDepth | integer | query | Ne |  |
| urlId | string | query | Ne |  |
| userId | string | query | Ne |  |
| anonUserId | string | query | Ne |  |
| contextUserId | string | query | Ne |  |
| hashTag | string | query | Ne |  |
| parentId | string | query | Ne |  |
| direction | string | query | Ne |  |

## Response

Vrne: [`GetComments200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetComments200Response.java)

## Primer

[inline-code-attrs-start title = 'Primer getComments'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // Konfiguriraj avtentikacijo z API ključem: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Odkomentirajte naslednjo vrstico, da nastavite predpono za API ključ, npr. "Token" (privzeto null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    Integer page = 56; // Integer | 
    Integer limit = 56; // Integer | 
    Integer skip = 56; // Integer | 
    Boolean asTree = true; // Boolean | 
    Integer skipChildren = 56; // Integer | 
    Integer limitChildren = 56; // Integer | 
    Integer maxTreeDepth = 56; // Integer | 
    String urlId = "urlId_example"; // String | 
    String userId = "userId_example"; // String | 
    String anonUserId = "anonUserId_example"; // String | 
    String contextUserId = "contextUserId_example"; // String | 
    String hashTag = "hashTag_example"; // String | 
    String parentId = "parentId_example"; // String | 
    SortDirections direction = SortDirections.fromValue("OF"); // SortDirections | 
    try {
      GetComments200Response result = apiInstance.getComments(tenantId)
            .page(page)
            .limit(limit)
            .skip(skip)
            .asTree(asTree)
            .skipChildren(skipChildren)
            .limitChildren(limitChildren)
            .maxTreeDepth(maxTreeDepth)
            .urlId(urlId)
            .userId(userId)
            .anonUserId(anonUserId)
            .contextUserId(contextUserId)
            .hashTag(hashTag)
            .parentId(parentId)
            .direction(direction)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getComments");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---