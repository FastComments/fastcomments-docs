Accoda un lavoro batch che sposta commenti, pagine e record correlati da un urlId o dominio a un altro.  
Le migrazioni **"urlId"** corrispondono solo all'esatto urlId. Le migrazioni **"domain"** trovano/sostituiscono il valore all'interno di ogni urlId e url.  
Esegui il polling **GET /api/v1/batch-jobs/{batchJobId}**: una volta completato, restituisce i risultati per migrazione e un **undoId**.

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Sì |  |

## Risposta

Restituisce: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/QueuePageMigrationJobAPIResponse.java)

## Esempio

[inline-code-attrs-start title = 'Esempio migratePages'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importa le classi:
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
    
    // Configura l'autorizzazione della chiave API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Decommenta la riga seguente per impostare un prefisso per la chiave API, ad esempio "Token" (predefinito a null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    MigrateAPIPagesData migrateAPIPagesData = new MigrateAPIPagesData(); // MigrateAPIPagesData | 
    try {
      QueuePageMigrationJobAPIResponse result = apiInstance.migratePages(tenantId, migrateAPIPagesData)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#migratePages");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]