## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| namespace | string | putanja | Da |  |
| component | string | putanja | Da |  |
| locale | string | upit | Ne |  |
| useFullTranslationIds | boolean | upit | Ne |  |

## Odgovor

Vraća: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetTranslationsResponse.java)

## Primer

[inline-code-attrs-start title = 'Primer getTranslations'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Uvoz klasa:
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