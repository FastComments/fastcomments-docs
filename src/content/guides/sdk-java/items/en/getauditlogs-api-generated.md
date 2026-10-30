## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | The tenant to read audit logs for. |
| limit | integer | query | No | How many logs to return. Max 10000, defaults to 1000. |
| skip | integer | query | No | How many logs to skip, for paging. |
| order | string | query | No | Sort direction on the event time. Defaults to DESC (newest first). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Only return events performed by this username. |
| ip | string | query | No | Only return events from this IP address. |
| crudType | string | query | No | Only return events of this type: c, r, u, d, or login. |
| resourceName | string | query | No | Only return events for this resource, e.g. User or Moderator. |
| targetId | string | query | No | Only return events whose affected object has this id. |
| target | string | query | No | Case-insensitive substring match on the affected object's label, e.g. a username or email. |
| includeManagedTenants | boolean | query | No | Also return events from tenants this tenant manages. |

## Response

Returns: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogsResponse.java)

## Example

[inline-code-attrs-start title = 'getAuditLogs Example'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String tenantId = "tenantId_example"; // String | The tenant to read audit logs for.
    Integer limit = 56; // Integer | How many logs to return. Max 10000, defaults to 1000.
    Integer skip = 56; // Integer | How many logs to skip, for paging.
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | Sort direction on the event time. Defaults to DESC (newest first).
    Long after = 56L; // Long | 
    Long before = 56L; // Long | 
    String username = "username_example"; // String | Only return events performed by this username.
    String ip = "ip_example"; // String | Only return events from this IP address.
    String crudType = "crudType_example"; // String | Only return events of this type: c, r, u, d, or login.
    String resourceName = "resourceName_example"; // String | Only return events for this resource, e.g. User or Moderator.
    String targetId = "targetId_example"; // String | Only return events whose affected object has this id.
    String target = "target_example"; // String | Case-insensitive substring match on the affected object's label, e.g. a username or email.
    Boolean includeManagedTenants = true; // Boolean | Also return events from tenants this tenant manages.
    try {
      GetAuditLogsResponse result = apiInstance.getAuditLogs(tenantId)
            .limit(limit)
            .skip(skip)
            .order(order)
            .after(after)
            .before(before)
            .username(username)
            .ip(ip)
            .crudType(crudType)
            .resourceName(resourceName)
            .targetId(targetId)
            .target(target)
            .includeManagedTenants(includeManagedTenants)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getAuditLogs");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]
