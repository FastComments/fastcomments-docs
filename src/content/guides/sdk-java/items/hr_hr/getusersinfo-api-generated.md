Skupne informacije o korisnicima za tenant. Za zadane userIds, vraća prikazne informacije iz User / SSOUser.
Koristi widget za komentare za obogaćivanje korisnika koji su se upravo pojavili putem događaja prisutnosti.
Nema konteksta stranice: privatnost se provodi jednako (privatni profili su maskirani).

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| ids | string | query | Da | userIds odvojeni zarezima. |

## Odgovor

Vraća: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersInfoResponse.java)

## Primjer

[inline-code-attrs-start title = 'Primjer getUsersInfo'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Uvezi klase:
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
    String ids = "ids_example"; // String | userIds odvojeni zarezima.
    try {
      PageUsersInfoResponse result = apiInstance.getUsersInfo(tenantId, ids)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getUsersInfo");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---