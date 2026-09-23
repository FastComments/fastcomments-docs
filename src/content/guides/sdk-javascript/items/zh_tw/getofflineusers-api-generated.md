Past commenters on the page who are NOT currently online. Sorted by displayName.  
過去在此頁面發表評論，但目前不在線上的評論者。依 displayName 排序。  

Use this after exhausting /users/online to render a "Members" section.  
在使用完 /users/online 之後，使用此方法來呈現「Members」區段。  

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
在 commenterName 上使用游標分頁：伺服器從部分 {tenantId, urlId, commenterName} 索引開始，從 afterName 往前以 $gt 前進，無需 $skip 成本。  

## Parameters

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## 回應

返回：[`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## 範例

[inline-code-attrs-start title = 'getOfflineUsers 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
async function fetchOfflineUsers(): Promise<void> {  
  const tenantId: string = "tenant_12345";  
  const urlId: string = "page_9876";  
  const afterName: string = "John Doe";  
  const afterUserId: string = "user_abc123";  

  const offlineResponse: PageUsersOfflineResponse = await getOfflineUsers(  
    tenantId,  
    urlId,  
    afterName,  
    afterUserId  
  );  

  console.log(offlineResponse);  
}  
[inline-code-end]  

---