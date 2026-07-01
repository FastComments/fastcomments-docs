## Parameters

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|-----------|----------|-------------|
| tenantId | string | query | Ja |  |
| sso | string | query | Nej |  |

## Response

Returnerer: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetBannedUsersCountResponse.java)

## Example

[inline-code-attrs-start title = 'getCounts Eksempel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importér klasser:
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
    String sso = "sso_example"; // String | 
    try {
      GetBannedUsersCountResponse result = apiInstance.getCounts(tenantId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Undtagelse ved kald af ModerationApi#getCounts");
      System.err.println("Statuskode: " + e.getCode());
      System.err.println("Årsag: " + e.getResponseBody());
      System.err.println("Svar‑headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]