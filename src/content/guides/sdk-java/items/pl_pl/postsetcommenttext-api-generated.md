## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| commentId | string | ścieżka | Tak |  |
| sso | string | zapytanie | Nie |  |

## Odpowiedź

Zwraca: [`SetCommentTextResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/SetCommentTextResponse.java)

## Przykład

[inline-code-attrs-start title = 'Przykład postSetCommentText'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importowanie klas:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.ModerationApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    ModerationApi apiInstance = new ModerationApi(defaultClient);
    String commentId = "commentId_example"; // String | 
    SetCommentTextParams setCommentTextParams = new SetCommentTextParams(); // SetCommentTextParams | 
    String sso = "sso_example"; // String | 
    try {
      SetCommentTextResponse result = apiInstance.postSetCommentText(commentId, setCommentTextParams)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ModerationApi#postSetCommentText");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]