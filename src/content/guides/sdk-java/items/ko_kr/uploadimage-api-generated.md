이미지 업로드 및 크기 조정

## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| sizePreset | string | query | 아니요 | 크기 프리셋: "Default" (1000x1000px) 또는 "CrossPlatform" (인기 기기에 대한 크기들을 생성) |
| urlId | string | query | 아니요 | 업로드가 발생하는 페이지의 ID(구성용) |

## 응답

반환: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/UploadImageResponse.java)

## 예제

[inline-code-attrs-start title = 'uploadImage 예제'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
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
    SizePreset sizePreset = SizePreset.fromValue("Default"); // SizePreset | 크기 프리셋: \"Default\" (1000x1000px) 또는 \"CrossPlatform\" (인기 기기용 크기를 생성)
    String urlId = "urlId_example"; // String | 업로드가 발생하는 페이지의 ID(구성용)
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