Бulk информация за потребители за наемател (tenant). За дадени userIds, връща информация за показване от User / SSOUser.
Използва се от коментарния widget за обогатяване на потребители, които току-що са се показали чрез presence събитие.
Без контекст на страница: поверителността се прилага еднородно (частните профили са маскирани).

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| ids | string | query | Да | Потребителски идентификатори, разделени със запетая. |

## Отговор

Връща: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersInfoResponse.java)

## Пример

[inline-code-attrs-start title = 'Пример за getUsersInfo'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String ids = "ids_example"; // String | Потребителски идентификатори, разделени със запетая.
    try {
      PageUsersInfoResponse result = apiInstance.getUsersInfo(tenantId, ids)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getUsersInfo");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]