## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| id | string | path | 예 |  |

## 응답

반환: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_patch_page_api_response.go)

## 예제

[inline-code-attrs-start title = 'PatchPage 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	id := "id_example" // string | 
	updateAPIPageData := *openapiclient.NewUpdateAPIPageData() // UpdateAPIPageData | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.PatchPage(context.Background(), id).TenantId(tenantId).UpdateAPIPageData(updateAPIPageData).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.PatchPage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `PatchPage`로부터의 응답: PatchPageAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.PatchPage`: %v\n", resp)
}
[inline-code-end]