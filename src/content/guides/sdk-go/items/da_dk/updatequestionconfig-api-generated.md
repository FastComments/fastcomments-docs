## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Svar

Returnerer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_api_empty_response.go)

## Eksempel

[inline-code-attrs-start title = 'UpdateQuestionConfig Eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	id := "id_example" // string | 
	updateQuestionConfigBody := *openapiclient.NewUpdateQuestionConfigBody() // UpdateQuestionConfigBody | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.UpdateQuestionConfig(context.Background(), id).TenantId(tenantId).UpdateQuestionConfigBody(updateQuestionConfigBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.UpdateQuestionConfig``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `UpdateQuestionConfig`: APIEmptyResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.UpdateQuestionConfig`: %v\n", resp)
}
[inline-code-end]