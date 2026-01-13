## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |

## Yanıt

Döndürür: [`CreateModerator200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_moderator_200_response.go)

## Örnek

[inline-code-attrs-start title = 'CreateModerator Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createModeratorBody := *openapiclient.NewCreateModeratorBody("Name_example", "Email_example") // CreateModeratorBody | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateModerator(context.Background()).TenantId(tenantId).CreateModeratorBody(createModeratorBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateModerator``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `CreateModerator`'den gelen yanıt: CreateModerator200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateModerator`: %v\n", resp)
}
[inline-code-end]

---