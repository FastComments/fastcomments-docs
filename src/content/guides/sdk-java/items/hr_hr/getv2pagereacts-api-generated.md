## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da |  |
| sso | string | query | Ne |  |
| includeUserInfo | string | query | Ne | Kada je postavljeno, vraća korisnike za svaki react id (avatari i korisnička imena). Samo "public" je prihvaćeno, što izostavlja privatne profile. |

## Odgovor

Vraća: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetV2PageReacts.java)

## Primjer

[inline-code-attrs-start title = 'Primjer getV2PageReacts'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // String | 
    String sso = "sso_example"; // String | 
    PageReactIncludeUserInfo includeUserInfo = PageReactIncludeUserInfo.fromValue("public"); // PageReactIncludeUserInfo | Kada je postavljeno, vraća korisnike za svaki react id (avatari i korisnička imena). Samo \"public\" je prihvaćeno, što izostavlja privatne profile.
    try {
      GetV2PageReacts result = apiInstance.getV2PageReacts(tenantId, urlId)
            .sso(sso)
            .includeUserInfo(includeUserInfo)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Izuzetak prilikom poziva PublicApi#getV2PageReacts");
      System.err.println("Kod statusa: " + e.getCode());
      System.err.println("Razlog: " + e.getResponseBody());
      System.err.println("Zaglavlja odgovora: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---