Téléverser et redimensionner une image

## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| sizePreset | string | query | Non | Préréglage de taille : "Default" (1000x1000px) ou "CrossPlatform" (crée des tailles pour les appareils populaires) |
| urlId | string | query | Non | ID de la page depuis laquelle le téléchargement a lieu, pour configurer |

## Réponse

Renvoie : [`UploadImageResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/UploadImageResponse.java)

## Exemple

[inline-code-attrs-start title = 'Exemple pour uploadImage'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importer les classes :
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
    File _file = new File("/path/to/file"); // File | 
    SizePreset sizePreset = SizePreset.fromValue("Default"); // SizePreset | Préréglage de taille : \"Default\" (1000x1000px) ou \"CrossPlatform\" (crée des tailles pour les appareils populaires)
    String urlId = "urlId_example"; // String | ID de la page depuis laquelle le téléchargement a lieu, pour configurer
    try {
      UploadImageResponse result = apiInstance.uploadImage(tenantId, _file)
            .sizePreset(sizePreset)
            .urlId(urlId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#uploadImage");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]