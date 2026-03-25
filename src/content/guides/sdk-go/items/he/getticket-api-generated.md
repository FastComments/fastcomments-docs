## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |
| userId | string | query | לא |  |

## תגובה

מחזיר: [`GetTicket200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_ticket_200_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-GetTicket'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTicket(context.Background(), id).TenantId(tenantId).UserId(userId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTicket``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`GetTicket`: GetTicket200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTicket`: %v\n", resp)
}
[inline-code-end]