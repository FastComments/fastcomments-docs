Качване и оразмеряване на изображение

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Предварително зададен размер: "Default" (1000x1000px) или "CrossPlatform" (създава размери за популярни устройства) |
| urlId | string | query | No | ID на страницата, от която се извършва качването, за конфигуриране |

## Отговор

Връща: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/UploadImageResponse.java)

## Пример

[inline-code-attrs-start title = 'Пример за uploadImage'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    File _file = new File("/path/to/file"); // File | 
    SizePreset sizePreset = SizePreset.fromValue("Default"); // SizePreset | Предварително зададен размер: \"Default\" (1000x1000px) или \"CrossPlatform\" (създава размери за популярни устройства)
    String urlId = "urlId_example"; // String | ID на страницата, от която се извършва качването, за конфигуриране
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

---