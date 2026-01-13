## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tag | string | path | Да |  |
| tenantId | string | query | Нет |  |

## Response

Возвращает: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_flag_comment_public_200_response.go)

## Пример

[inline-code-attrs-start title = 'Пример DeleteHashTag'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tag := "tag_example" // string | 
	tenantId := "tenantId_example" // string |  (необязательно)
	deleteHashTagRequest := *openapiclient.NewDeleteHashTagRequest() // DeleteHashTagRequest |  (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.DeleteHashTag(context.Background(), tag).TenantId(tenantId).DeleteHashTagRequest(deleteHashTagRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.DeleteHashTag``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// ответ от `DeleteHashTag`: FlagCommentPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.DeleteHashTag`: %v\n", resp)
}
[inline-code-end]