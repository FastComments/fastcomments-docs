---
An `AuditLog` 是一個物件，代表對有權使用此功能的租戶所記錄的稽核事件。

The structure for the AuditLog object is as follows:

[inline-code-attrs-start title = 'AuditLog 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string;
    userId?: string;
    username?: string;
    resourceName: string;
    crudType: 'c' | 'r' | 'u' | 'd' | 'login';
    from: string;
    url?: string;
    ip?: string;
    when: string;
    description?: string;
    serverStartDate: string;
    objectDetails?: object;
}
[inline-code-end]

稽核日誌是不可變的。也無法手動寫入。FastComments.com 可能是唯一能決定何時寫入稽核日誌的實體。不過，你可以透過此 API 讀取它。

Events in the audit log expire after two years.

---