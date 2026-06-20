Листа страница за tenant. Користи се од стране FChat десктоп клијента за попуњавање листе соба.
Захтијева да је `enableFChat` постављен на true на решеном прилагођеном конфигу за сваку страницу.
Странице које захтијевају SSO филтрирају се у складу са приступом групе корисника који прави захтев.

## Parameters

| Име | Тип | Локација | Потребно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| cursor | string | query | Не | Непрозирни курсор пагинације који се враћа као `nextCursor` из претходног захтева. Везан за исти `sortBy`. |
| limit | integer | query | Не | 1..200, подразумевано 50 |
| q | string | query | Не | Опциони филтер префикса наслова који није осетљив на велика/мала слова. |
| sortBy | string | query | Не | Редослијед сортирања. `updatedAt` (подразумевано, најновије прво), `commentCount` (највише коментара прво), или `title` (азбучно). |
| hasComments | boolean | query | Не | Ако је true, враћају се само странице са најмање једним коментаром. |

## Response

Враћа: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_public_pages_response.go)

## Пример

[inline-code-attrs-start title = 'Пример GetPagesPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	cursor := "cursor_example" // string | Непрозирни курсор пагинације који се враћа као `nextCursor` из претходног захтева. Везан за исти `sortBy`. (опционо)
	limit := int32(56) // int32 | 1..200, подразумевано 50 (опционо)
	q := "q_example" // string | Опциони филтер префикса наслова који није осетљив на велика/мала слова. (опционо)
	sortBy := openapiclient.PagesSortBy("updatedAt") // PagesSortBy | Редослијед сортирања. `updatedAt` (подразумевано, најновије прво), `commentCount` (највише коментара прво), или `title` (азбучно). (опционо)
	hasComments := true // bool | Ако је true, враћају се само странице са најмање једним коментаром. (опционо)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetPagesPublic(context.Background(), tenantId).Cursor(cursor).Limit(limit).Q(q).SortBy(sortBy).HasComments(hasComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetPagesPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// одговор од `GetPagesPublic`: GetPublicPagesResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetPagesPublic`: %v\n", resp)
}
[inline-code-end]