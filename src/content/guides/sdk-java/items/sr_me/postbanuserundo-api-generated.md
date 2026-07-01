## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| sso | string | query | No |  |

## Odgovor

Vraća: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/APIEmptyResponse.java)

## Primjer

[inline-code-attrs-start title = 'postBanUserUndo Primjer'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
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
    String tenantId = "tenantId_example"; // String | 
    BanUserUndoParams banUserUndoParams = new BanUserUndoParams(); // BanUserUndoParams | 
    String sso = "sso_example"; // String | 
    try {
      APIEmptyResponse result = apiInstance.postBanUserUndo(tenantId, banUserUndoParams)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      // Exception when calling ModerationApi#postBanUserUndo
      System.err.println("Exception when calling ModerationApi#postBanUserUndo");
      // Status code: 
      System.err.println("Status code: " + e.getCode());
      // Reason: 
      System.err.println("Reason: " + e.getResponseBody());
      // Response headers: 
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]