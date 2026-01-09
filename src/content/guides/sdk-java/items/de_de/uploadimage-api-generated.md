Bild hochladen und skalieren

## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| sizePreset | string | query | Nein | Size preset: "Default" (1000x1000px) or "CrossPlatform" (creates sizes for popular devices) |
| urlId | string | query | Nein | Seiten-ID, von der der Upload erfolgt, zur Konfiguration |

## Antwort

Gibt zurück: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/UploadImageResponse.java)

## Beispiel

[inline-code-attrs-start title = 'uploadImage Beispiel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Klassen importieren:
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
    SizePreset sizePreset = SizePreset.fromValue("Default"); // SizePreset | Voreinstellung für die Größe: \"Default\" (1000x1000px) oder \"CrossPlatform\" (erstellt Größen für gängige Geräte)
    String urlId = "urlId_example"; // String | Seiten-ID, von der der Upload erfolgt, zur Konfiguration
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