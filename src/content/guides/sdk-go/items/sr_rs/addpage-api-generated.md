---
## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |

## Одговор

Враћа: [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_add_page_api_response.go)

## Пример

[inline-code-attrs-start title = 'Пример додавања странице'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createAPIPageData := *openapiclient.NewCreateAPIPageData("Title_example", "Url_example", "UrlId_example") // CreateAPIPageData | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.AddPage(context.Background()).TenantId(tenantId).CreateAPIPageData(createAPIPageData).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.AddPage``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// одговор од `AddPage`: AddPageAPIResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.AddPage`: %v\n", resp)
}
[inline-code-end]

---