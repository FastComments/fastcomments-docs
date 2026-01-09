## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| page | integer | query | Нет |  |
| limit | integer | query | Нет |  |
| skip | integer | query | Нет |  |
| asTree | boolean | query | Нет |  |
| skipChildren | integer | query | Нет |  |
| limitChildren | integer | query | Нет |  |
| maxTreeDepth | integer | query | Нет |  |
| urlId | string | query | Нет |  |
| userId | string | query | Нет |  |
| anonUserId | string | query | Нет |  |
| contextUserId | string | query | Нет |  |
| hashTag | string | query | Нет |  |
| parentId | string | query | Нет |  |
| direction | string | query | Нет |  |

## Ответ

Возвращает: [`GetComments200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_200_response.go)

## Пример

[inline-code-attrs-start title = 'Пример GetComments'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (необязательно)
	limit := int32(56) // int32 |  (необязательно)
	skip := int32(56) // int32 |  (необязательно)
	asTree := true // bool |  (необязательно)
	skipChildren := int32(56) // int32 |  (необязательно)
	limitChildren := int32(56) // int32 |  (необязательно)
	maxTreeDepth := int32(56) // int32 |  (необязательно)
	urlId := "urlId_example" // string |  (необязательно)
	userId := "userId_example" // string |  (необязательно)
	anonUserId := "anonUserId_example" // string |  (необязательно)
	contextUserId := "contextUserId_example" // string |  (необязательно)
	hashTag := "hashTag_example" // string |  (необязательно)
	parentId := "parentId_example" // string |  (необязательно)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (необязательно)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetComments(context.Background()).TenantId(tenantId).Page(page).Limit(limit).Skip(skip).AsTree(asTree).SkipChildren(skipChildren).LimitChildren(limitChildren).MaxTreeDepth(maxTreeDepth).UrlId(urlId).UserId(userId).AnonUserId(anonUserId).ContextUserId(contextUserId).HashTag(hashTag).ParentId(parentId).Direction(direction).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetComments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// ответ от `GetComments`: GetComments200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetComments`: %v\n", resp)
}
[inline-code-end]