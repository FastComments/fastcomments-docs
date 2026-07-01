req  
请求  
tenantId  
urlId  
userIdWS  

## 参数  

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| urlId | string | 是 |  |
| userIdWS | string | 是 |  |
| startTime | number | 是 |  |
| endTime | number | 否 |  |

## 响应  

返回: [`GetEventLogResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetEventLogResponse1.ts)

## 示例  

[inline-code-attrs-start title = 'getEventLog 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
(async () => {  
    const tenantId: string = "tenant_9876";  
    const urlId: string = "page_54321";  
    const userIdWS: string = "ws_user_1122";  
    const startTime: number = Date.now() - 7 * 24 * 60 * 60 * 1000; // 一周前  
    const endTime: number = Date.now();  

    const log: GetEventLogResponse1 = await getEventLog(tenantId, urlId, userIdWS, startTime, endTime);  
    const recentLog: GetEventLogResponse1 = await getEventLog(tenantId, urlId, userIdWS, startTime);  
})();  
[inline-code-end]