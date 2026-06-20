## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| namespace | string | path | Tak |  |
| component | string | path | Tak |  |
| locale | string | query | Nie |  |
| useFullTranslationIds | boolean | query | Nie |  |

## Odpowiedź

Zwraca: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetTranslationsResponse.java)

## Przykład

[inline-code-attrs-start title = 'Przykład getTranslations'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import klas:
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
    String namespace = "namespace_example"; // String | 
    String component = "component_example"; // String | 
    String locale = "locale_example"; // String | 
    Boolean useFullTranslationIds = true; // Boolean | 
    try {
      GetTranslationsResponse result = apiInstance.getTranslations(namespace, component)
            .locale(locale)
            .useFullTranslationIds(useFullTranslationIds)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getTranslations");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---