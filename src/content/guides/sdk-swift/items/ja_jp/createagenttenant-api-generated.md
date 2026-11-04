人間のサインアップなしで、AIエージェント用の新しいトライアルアカウントを作成します。この呼び出しにはAPIキーは不要です。

レスポンスには、テナントID、REST APIおよびMCPサーバーに対して即座に使用可能なAPIキー、そしてクレームURLが含まれます。クレームURLは、あなたが作業している依頼者(人間)に渡してください。FastCommentsにログインした状態でこのURLを開くと、アカウントがその人に紐付けられます。クレームされていないアカウントとそのキーは、作成から72時間後に削除されます。クレームされるまで、アカウントには標準のトライアル制限が適用されます。

## レスポンス

戻り値: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateAgentTenantResponse.swift)

## 例

[inline-code-attrs-start title = 'createAgentTenant の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let createAgentTenantBody = CreateAgentTenantBody(name: "name_example", domains: ["domains_example"], agent: "agent_example") // CreateAgentTenantBody | 

DefaultAPI.createAgentTenant(createAgentTenantBody: createAgentTenantBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]