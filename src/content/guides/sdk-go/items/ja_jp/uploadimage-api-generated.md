画像のアップロードとリサイズ

## パラメータ

| 名前 | Type | 位置 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | パス | はい |  |
| sizePreset | string | クエリ | いいえ | サイズプリセット: "Default" (1000x1000px) または "CrossPlatform" (一般的なデバイス向けのサイズを作成) |
| urlId | string | クエリ | いいえ | アップロードが行われているページのID、設定用 |

## レスポンス

返却: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_upload_image_response.go)

## 例

[inline-code-attrs-start title = 'UploadImage の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	sizePreset := openapiclient.SizePreset("Default") // SizePreset | サイズプリセット: \"Default\" (1000x1000px) または \"CrossPlatform\" (一般的なデバイス向けのサイズを作成) (オプション)
	urlId := "urlId_example" // string | アップロードが行われているページのID、設定用 (オプション)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.UploadImage(context.Background(), tenantId).File(file).SizePreset(sizePreset).UrlId(urlId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.UploadImage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `UploadImage` からのレスポンス: UploadImageResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.UploadImage`: %v\n", resp)
}
[inline-code-end]

---