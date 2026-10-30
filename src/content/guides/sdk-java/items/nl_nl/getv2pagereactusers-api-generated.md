## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | pad | Ja |  |
| urlId | string | query | Ja |  |
| id | string | query | Ja |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetV2PageReactUsersResponse.java)

## Voorbeeld

[inline-code-attrs-start title = 'getV2PageReactUsers Voorbeeld'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Klassen importeren:
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
    String urlId = "urlId_example"; // String | 
    String id = "id_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      GetV2PageReactUsersResponse result = apiInstance.getV2PageReactUsers(tenantId, urlId, id)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exceptie bij het aanroepen van PublicApi#getV2PageReactUsers");
      System.err.println("Statuscode: " + e.getCode());
      System.err.println("Reden: " + e.getResponseBody());
      System.err.println("Responsheaders: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---