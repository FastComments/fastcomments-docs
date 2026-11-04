Sætter et batch-job i kø, der tilbagefører hver migration fra ét migrate-pages-job, nyeste først. Tilgængelig i 90 dage.
Poster, der er ændret siden migrationen, efterlades urørt, så en senere redigering aldrig bliver overskrevet.
Poll GET /api/v1/batch-jobs/{batchJobId} for at se antallet af gendannede/sprunne-over.

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| undoId | string | path | Ja |  |

## Svar

Returnerer: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/QueuePageMigrationJobAPIResponse.java)

## Eksempel

[inline-code-attrs-start title = 'undoPageMigrations Eksempel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importér klasser:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.auth.*;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");
    
    // Konfigurér API-nøgleautorisation: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Udkommentér følgende linje for at sætte et præfiks for API-nøglen, f.eks. "Token" (standard er null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String undoId = "undoId_example"; // String | 
    try {
      QueuePageMigrationJobAPIResponse result = apiInstance.undoPageMigrations(tenantId, undoId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception ved kald af DefaultApi#undoPageMigrations");
      System.err.println("Statuskode: " + e.getCode());
      System.err.println("Årsag: " + e.getResponseBody());
      System.err.println("Svarheadere: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]