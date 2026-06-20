## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| page | integer | query | Nein |  |
| limit | integer | query | Nein |  |
| skip | integer | query | Nein |  |
| asTree | boolean | query | Nein |  |
| skipChildren | integer | query | Nein |  |
| limitChildren | integer | query | Nein |  |
| maxTreeDepth | integer | query | Nein |  |
| urlId | string | query | Nein |  |
| userId | string | query | Nein |  |
| anonUserId | string | query | Nein |  |
| contextUserId | string | query | Nein |  |
| hashTag | string | query | Nein |  |
| parentId | string | query | Nein |  |
| direction | string | query | Nein |  |
| fromDate | integer | query | Nein |  |
| toDate | integer | query | Nein |  |

## Antwort

Gibt zurück: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/APIGetCommentsResponse.java)

## Beispiel

[inline-code-attrs-start title = 'getComments Beispiel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Klassen importieren:
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
    
    // API-Schlüssel-Authorisierung konfigurieren: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Kommentieren Sie die folgende Zeile aus, um ein Präfix für den API-Schlüssel zu setzen, z. B. "Token" (Standard: null)
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