## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| banEmail | boolean | query | No |  |
| banEmailDomain | boolean | query | No |  |
| banIP | boolean | query | No |  |
| deleteAllUsersComments | boolean | query | No |  |
| bannedUntil | string | query | No |  |
| isShadowBan | boolean | query | No |  |
| updateId | string | query | No |  |
| banReason | string | query | No |  |
| sso | string | query | No |  |

## Réponse

Retourne : [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_ban_user_from_comment_result.go)

## Exemple

[inline-code-attrs-start title = 'Exemple PostBanUserFromComment'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	commentId := "commentId_example" // string | 
	banEmail := true // bool |  (optionnel)
	banEmailDomain := true // bool |  (optionnel)
	banIP := true // bool |  (optionnel)
	deleteAllUsersComments := true // bool |  (optionnel)
	bannedUntil := "bannedUntil_example" // string |  (optionnel)
	isShadowBan := true // bool |  (optionnel)
	updateId := "updateId_example" // string |  (optionnel)
	banReason := "banReason_example" // string |  (optionnel)
	sso := "sso_example" // string |  (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.PostBanUserFromComment(context.Background(), commentId).TenantId(tenantId).BanEmail(banEmail).BanEmailDomain(banEmailDomain).BanIP(banIP).DeleteAllUsersComments(deleteAllUsersComments).BannedUntil(bannedUntil).IsShadowBan(isShadowBan).UpdateId(updateId).BanReason(banReason).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Erreur lors de l'appel `ModerationAPI.PostBanUserFromComment`` : %v\n", err)
		fmt.Fprintf(os.Stderr, "Réponse HTTP complète : %v\n", r)
	}
	// réponse de `PostBanUserFromComment` : BanUserFromCommentResult
	fmt.Fprintf(os.Stdout, "Réponse de `ModerationAPI.PostBanUserFromComment` : %v\n", resp)
}
[inline-code-end]