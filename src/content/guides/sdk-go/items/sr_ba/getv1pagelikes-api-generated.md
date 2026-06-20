## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | путања | Да |  |
| urlId | string | упит | Да |  |

## Одговор

Враћа: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_v1_page_likes.go)

## Пример

[inline-code-attrs-start title = 'Пример GetV1PageLikes'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	urlId := "urlId_example" // string | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetV1PageLikes(context.Background(), tenantId).UrlId(urlId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetV1PageLikes``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// одговор од `GetV1PageLikes`: GetV1PageLikes
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetV1PageLikes`: %v\n", resp)
}
[inline-code-end]