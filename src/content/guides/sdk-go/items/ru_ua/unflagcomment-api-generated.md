## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| userId | string | query | Нет |  |
| anonUserId | string | query | Нет |  |

## Ответ

Возвращает: [`FlagComment200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_flag_comment_200_response.go)

## Пример

[inline-code-attrs-start title = 'Пример UnFlagComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userId := "userId_example" // string |  (необязательно)
	anonUserId := "anonUserId_example" // string |  (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.UnFlagComment(context.Background(), id).TenantId(tenantId).UserId(userId).AnonUserId(anonUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.UnFlagComment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// ответ от `UnFlagComment`: FlagComment200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.UnFlagComment`: %v\n", resp)
}
[inline-code-end]