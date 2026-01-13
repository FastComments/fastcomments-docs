## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## 응답

반환: [`CreateEmailTemplate200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_create_email_template_200_response.go)

## 예제

[inline-code-attrs-start title = 'CreateEmailTemplate 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	createEmailTemplateBody := *openapiclient.NewCreateEmailTemplateBody("EmailTemplateId_example", "DisplayName_example", "Ejs_example") // CreateEmailTemplateBody | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.CreateEmailTemplate(context.Background()).TenantId(tenantId).CreateEmailTemplateBody(createEmailTemplateBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.CreateEmailTemplate``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `CreateEmailTemplate`의 응답: CreateEmailTemplate200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.CreateEmailTemplate`: %v\n", resp)
}
[inline-code-end]

---