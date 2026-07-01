## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|-----------|----------|-------------|
| tenantId | string | query | Yes |  |
| batchJobId | string | query | No |  |
| sso | string | query | No |  |

## Svar

Returnerer: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/ModerationExportStatusResponse.java)

## Eksempel

[inline-code-attrs-start title = 'getApiExportStatus Eksempel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String tenantId = "tenantId_example"; // String | 
    String batchJobId = "batchJobId_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      ModerationExportStatusResponse result = apiInstance.getApiExportStatus(tenantId)
            .batchJobId(batchJobId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Undtagelse ved kald af ModerationApi#getApiExportStatus");
      System.err.println("Statuskode: " + e.getCode());
      System.err.println("Årsag: " + e.getResponseBody());
      System.err.println("Svaroverskrifter: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]