## Parametri

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
| fromDate | integer | query | Ne |  |
| toDate | integer | query | Ne |  |

## Odgovor

Vraća: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/APIGetCommentsResponse.java)

## Primjer

[inline-code-attrs-start title = 'getComments Primjer'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
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
    
    // Configure API key authorization: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Uncomment the following line to set a prefix for the API key, e.g. "Token" (defaults to null)
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
    Long fromDate = 56L; // Long | 
    Long toDate = 56L; // Long | 
    try {
      APIGetCommentsResponse result = apiInstance.getComments(tenantId)
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
            .fromDate(fromDate)
            .toDate(toDate)
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