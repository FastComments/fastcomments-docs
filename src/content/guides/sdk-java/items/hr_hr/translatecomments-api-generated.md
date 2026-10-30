Translates the given comments into the widget locale. Only available when the matching widget
customization rule has Comment Translation set to Automatic or Manual.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| sso | string | query | No |  |
| locale | string | query | No |  |

## Odgovor

Vraća: [`CommentTranslationsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CommentTranslationsResponse.java)

## Primjer

[inline-code-attrs-start title = 'Primjer translateComments'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    CommentTranslationsRequest commentTranslationsRequest = new CommentTranslationsRequest(); // CommentTranslationsRequest | 
    String sso = "sso_example"; // String | 
    String locale = "locale_example"; // String | 
    try {
      CommentTranslationsResponse result = apiInstance.translateComments(tenantId, urlId, commentTranslationsRequest)
            .sso(sso)
            .locale(locale)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#translateComments");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]