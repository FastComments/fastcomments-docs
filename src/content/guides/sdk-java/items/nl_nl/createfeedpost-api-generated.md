## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| broadcastId | string | query | No |  |
| isLive | boolean | query | No |  |
| doSpamCheck | boolean | query | No |  |
| skipDupCheck | boolean | query | No |  |

## Respons

Geeft terug: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateFeedPost200Response.java)

## Voorbeeld

[inline-code-attrs-start title = 'createFeedPost Voorbeeld'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importeer klassen:
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
    
    // Configureer API-sleutelautorisatie: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Haal de commentaar weg van de volgende regel om een voorvoegsel voor de API-sleutel in te stellen, bijv. "Token" (standaard null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    CreateFeedPostParams createFeedPostParams = new CreateFeedPostParams(); // CreateFeedPostParams | 
    String broadcastId = "broadcastId_example"; // String | 
    Boolean isLive = true; // Boolean | 
    Boolean doSpamCheck = true; // Boolean | 
    Boolean skipDupCheck = true; // Boolean | 
    try {
      CreateFeedPost200Response result = apiInstance.createFeedPost(tenantId, createFeedPostParams)
            .broadcastId(broadcastId)
            .isLive(isLive)
            .doSpamCheck(doSpamCheck)
            .skipDupCheck(skipDupCheck)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#createFeedPost");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]