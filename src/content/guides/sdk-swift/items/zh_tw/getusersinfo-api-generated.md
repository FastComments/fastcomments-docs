批次取得租戶的使用者資訊。給定 userIds，回傳來自 User / SSOUser 的顯示資訊。
由評論小工具使用，以補充剛透過 presence 事件出現的使用者資訊。
無頁面上下文：隱私會統一強制執行（私人檔案會被遮蔽）。

## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| ids | string | query | 是 | 逗號分隔的 userIds。 |

## 回應

回傳：[`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersInfoResponse.swift)

## 範例

[inline-code-attrs-start title = 'getUsersInfo 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式碼範例仍為測試版。若有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let ids = "ids_example" // String | 以逗號分隔的 userIds。

PublicAPI.getUsersInfo(tenantId: tenantId, ids: ids) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---