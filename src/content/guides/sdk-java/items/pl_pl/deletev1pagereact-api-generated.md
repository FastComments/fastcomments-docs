## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|------------|----------|------|
| tenantId | string | path | Tak |  |
| urlId | string | query | Tak |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateV1PageReact.java)

## Przykład

[inline-code-attrs-start title = 'deleteV1PageReact Przykład'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importuj klasy:
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
    try {
      CreateV1PageReact result = apiInstance.deleteV1PageReact(tenantId, urlId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Wyjątek podczas wywoływania PublicApi#deleteV1PageReact");
      System.err.println("Kod statusu: " + e.getCode());
      System.err.println("Powód: " + e.getResponseBody());
      System.err.println("Nagłówki odpowiedzi: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---