## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| includeByUserIdAndEmail | boolean | query | Nej |  |
| includeByIP | boolean | query | Nej |  |
| includeByEmailDomain | boolean | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/BulkPreBanSummary.java)

## Eksempel

[inline-code-attrs-start title = 'postBulkPreBanSummary Eksempel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importer klasser:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.ModerationApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    ModerationApi apiInstance = new ModerationApi(defaultClient);
    BulkPreBanParams bulkPreBanParams = new BulkPreBanParams(); // BulkPreBanParams | 
    Boolean includeByUserIdAndEmail = true; // Boolean | 
    Boolean includeByIP = true; // Boolean | 
    Boolean includeByEmailDomain = true; // Boolean | 
    String sso = "sso_example"; // String | 
    try {
      BulkPreBanSummary result = apiInstance.postBulkPreBanSummary(bulkPreBanParams)
            .includeByUserIdAndEmail(includeByUserIdAndEmail)
            .includeByIP(includeByIP)
            .includeByEmailDomain(includeByEmailDomain)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ModerationApi#postBulkPreBanSummary");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---