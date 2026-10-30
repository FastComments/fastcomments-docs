Traduit les commentaires fournis dans la langue du widget. Disponible uniquement lorsque la règle de personnalisation du widget correspondante a la traduction des commentaires définie sur Automatique ou Manuel.

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui |  |
| sso | string | query | Non |  |
| locale | string | query | Non |  |

## Réponse

Returns: [`CommentTranslationsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CommentTranslationsResponse.java)

## Exemple

[inline-code-attrs-start title = 'Exemple translateComments'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importer les classes:
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