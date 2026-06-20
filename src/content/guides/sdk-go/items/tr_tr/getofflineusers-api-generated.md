Sayfadaki ancak şu anda çevrimiçi olmayan önceki yorum yapan kullanıcılar. displayName'e göre sıralanır.
/users/online'ı tükettikten sonra "Members" bölümünü render etmek için bunu kullanın.
commenterName üzerinde imleç (cursor) sayfalandırması: sunucu, kısmi {tenantId, urlId, commenterName} indeksinde afterName'den itibaren $gt ile ileri doğru yürür, $skip maliyeti yok.

## Parametreler

| Ad | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir). |
| afterName | string | query | No | İmleç: önceki yanıttan nextAfterName değerini aktarın. |
| afterUserId | string | query | No | İmleç eşitleyicisi: önceki yanıttan nextAfterUserId değerini aktarın. afterName ayarlandığında, isim eşitlikleri nedeniyle girdilerin düşmemesi için gereklidir. |

## Yanıt

Döndürür: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_page_users_offline_response.go)

## Örnek

[inline-code-attrs-start title = 'GetOfflineUsers Örneği'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | Sayfa URL tanımlayıcısı (sunucu tarafında temizlenir).
	afterName := "afterName_example" // string | İmleç: önceki yanıttan nextAfterName değerini aktarın. (isteğe bağlı)
	afterUserId := "afterUserId_example" // string | İmleç eşitleyicisi: önceki yanıttan nextAfterUserId değerini aktarın. afterName ayarlandığında, isim eşitlikleri nedeniyle girdilerin düşmemesi için gereklidir. (isteğe bağlı)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetOfflineUsers(context.Background(), tenantId).UrlId(urlId).AfterName(afterName).AfterUserId(afterUserId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetOfflineUsers``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetOfflineUsers`'ten gelen yanıt: PageUsersOfflineResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetOfflineUsers`: %v\n", resp)
}
[inline-code-end]

---