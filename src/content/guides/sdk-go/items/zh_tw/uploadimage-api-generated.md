上傳並調整影像大小

## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| sizePreset | string | query | 否 | 大小預設："Default" (1000x1000px) 或 "CrossPlatform" (為熱門裝置建立尺寸) |
| urlId | string | query | 否 | 上傳發生的頁面 ID，用於設定 |

## 回應

回傳: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_upload_image_response.go)

## 範例

[inline-code-attrs-start title = 'UploadImage 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	file := os.NewFile(1234, "some_file") // *os.File | 
	sizePreset := openapiclient.SizePreset("Default") // SizePreset | 大小預設值: \"Default\" (1000x1000px) 或 \"CrossPlatform\" (為熱門裝置建立尺寸) (可選)
	urlId := "urlId_example" // string | 上傳發生的頁面 ID，用於設定 (可選)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UploadImage(context.Background(), tenantId).File(file).SizePreset(sizePreset).UrlId(urlId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UploadImage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 來自 `UploadImage` 的回應: UploadImageResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UploadImage`: %v\n", resp)
}
[inline-code-end]