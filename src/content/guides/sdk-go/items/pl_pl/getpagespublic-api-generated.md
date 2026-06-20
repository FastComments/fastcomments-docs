Wylistuj strony dla najemcy. Używane przez klienta desktopowego FChat do wypełnienia swojej listy pokoi.
Wymaga, aby `enableFChat` było ustawione na true w rozstrzygniętej konfiguracji niestandardowej dla każdej strony.
Strony, które wymagają SSO, są filtrowane względem dostępu grupowego użytkownika wysyłającego żądanie.

## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Niejawny kursor paginacji zwrócony jako `nextCursor` z poprzedniego żądania. Powiązany z tym samym `sortBy`. |
| limit | integer | query | No | 1..200, domyślnie 50 |
| q | string | query | No | Opcjonalny filtr prefiksu tytułu niezależny od wielkości liter. |
| sortBy | string | query | No | Kolejność sortowania. `updatedAt` (domyślnie, najnowsze pierwsze), `commentCount` (najpierw najwięcej komentarzy), lub `title` (alfabetycznie). |
| hasComments | boolean | query | No | Jeśli true, zwróć tylko strony mające co najmniej jeden komentarz. |

## Odpowiedź

Zwraca: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_public_pages_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład GetPagesPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	cursor := "cursor_example" // string | Niejawny kursor paginacji zwrócony jako `nextCursor` z poprzedniego żądania. Powiązany z tym samym `sortBy`. (opcjonalne)
	limit := int32(56) // int32 | 1..200, domyślnie 50 (opcjonalne)
	q := "q_example" // string | Opcjonalny filtr prefiksu tytułu niezależny od wielkości liter. (opcjonalne)
	sortBy := openapiclient.PagesSortBy("updatedAt") // PagesSortBy | Kolejność sortowania. `updatedAt` (domyślnie, najnowsze pierwsze), `commentCount` (najpierw najwięcej komentarzy), lub `title` (alfabetycznie). (opcjonalne)
	hasComments := true // bool | Jeśli true, zwróć tylko strony mające co najmniej jeden komentarz. (opcjonalne)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetPagesPublic(context.Background(), tenantId).Cursor(cursor).Limit(limit).Q(q).SortBy(sortBy).HasComments(hasComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetPagesPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `GetPagesPublic`: GetPublicPagesResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetPagesPublic`: %v\n", resp)
}
[inline-code-end]