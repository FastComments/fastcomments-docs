An `AuditLog` 是一個物件，代表具有此功能存取權限的租戶的稽核事件。

AuditLog 物件的結構如下：

[inline-code-attrs-start title = 'AuditLog 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string;
    /** Who performed the event. **/
    userId?: string;
    username?: string;
    resourceName: string;
    crudType: 'c' | 'r' | 'u' | 'd' | 'login';
    from: string;
    url?: string;
    ip?: string;
    /** The browser that performed the event, when it came from one. **/
    ua?: string;
    /** A hash of the session the event came from, for correlating one person's actions. Never the session itself. **/
    sIdHashed?: string;
    when: string;
    description?: string;
    serverStartDate: string;
    /** The id of the object the event was performed on, as opposed to who performed it. **/
    targetId?: string;
    /** A human-readable label for that object, e.g. "jsmith (jsmith@example.com)". **/
    targetLabel?: string;
    objectDetails?: object;
}
[inline-code-end]

`targetId` 與 `targetLabel` 描述事件執行的對象；`userId` 與 `username` 描述執行者。對於更新，`objectDetails.changes` 包含一個 `{field: {from, to}}` 的映射，說明實際變更的內容。

稽核日誌是不可變的，也無法手動寫入。FastComments.com 僅能決定何時寫入稽核日誌。然而，您可以透過此 API 讀取它。

稽核日誌中的事件會在兩年後過期。