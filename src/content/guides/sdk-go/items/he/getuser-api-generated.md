## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |

## תגובה

מחזיר: [`GetUser200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_user_200_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמת GetUser'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // מחרוזת | 
	id := "id_example" // מחרוזת | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetUser(context.Background(), id).TenantId(tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetUser``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`GetUser`: GetUser200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetUser`: %v\n", resp)
}
[inline-code-end]