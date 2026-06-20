Изброява страници за наемател. Използва се от десктоп клиента FChat за попълване на списъка със стаи.
Изисква `enableFChat` да бъде true в разрешената персонализирана конфигурация за всяка страница.
Страниците, които изискват SSO, се филтрират въз основа на груповия достъп на потребителя, правещ заявката.

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозрачен курсор за страниране, върнат като `nextCursor` от предишна заявка. Свързан със същия `sortBy`. |
| limit | integer | query | No | 1..200, по подразбиране 50 |
| q | string | query | No | Незадължителен филтър за префикс на заглавието, нечувствителен към регистър. |
| sortBy | string | query | No | Ред на сортиране. `updatedAt` (по подразбиране, най-новите първи), `commentCount` (първо най-много коментари), или `title` (азбучно). |
| hasComments | boolean | query | No | Ако е true, връща само страници с поне един коментар. |

## Отговор

Връща: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_public_pages_response.go)

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
	cursor := "cursor_example" // string | Непрозрачен курсор за страниране, върнат като `nextCursor` от предишна заявка. Свързан със същия `sortBy`. (по избор)
	limit := int32(56) // int32 | 1..200, по подразбиране 50 (по избор)
	q := "q_example" // string | Незадължителен филтър за префикс на заглавието, нечувствителен към регистър. (по избор)
	sortBy := openapiclient.PagesSortBy("updatedAt") // PagesSortBy | Ред на сортиране. `updatedAt` (по подразбиране, най-новите първи), `commentCount` (първо най-много коментари), или `title` (азбучно). (по избор)
	hasComments := true // bool | Ако е true, връща само страници с поне един коментар. (по избор)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetPagesPublic(context.Background(), tenantId).Cursor(cursor).Limit(limit).Q(q).SortBy(sortBy).HasComments(hasComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetPagesPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// отговор от `GetPagesPublic`: GetPublicPagesResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetPagesPublic`: %v\n", resp)
}
[inline-code-end]