Предишни коментатори на страницата, които не са в момента онлайн. Сортирани по displayName.
Използвайте това след изчерпване на /users/online, за да визуализирате секция "Членове".
Курсорно пагиниране по commenterName: сървърът обхожда частичния {tenantId, urlId, commenterName}
индекс от afterName напред чрез $gt, без разход за $skip.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор на URL на страницата (изчистен на страната на сървъра). |
| afterName | string | query | Не | Курсор: подайте nextAfterName от предния отговор. |
| afterUserId | string | query | Не | Допълнителен курсор за разделяне при равенство: подайте nextAfterUserId от предния отговор. Задължително когато afterName е зададен, за да не се изпуснат записи при едно и също име. |

## Response

Връща: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOfflineResponse.java)

## Example

[inline-code-attrs-start title = 'Пример за getOfflineUsers'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Импортиране на класове:
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
    String urlId = "urlId_example"; // String | Идентификатор на URL на страницата (изчистен на страната на сървъра).
    String afterName = "afterName_example"; // String | Курсор: подайте nextAfterName от предния отговор.
    String afterUserId = "afterUserId_example"; // String | Допълнителен курсор за разделяне при равенство: подайте nextAfterUserId от предния отговор. Задължително когато afterName е зададен, за да не се изпуснат записи при едно и също име.
    try {
      PageUsersOfflineResponse result = apiInstance.getOfflineUsers(tenantId, urlId)
            .afterName(afterName)
            .afterUserId(afterUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getOfflineUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]