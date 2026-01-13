---
一个 `AuditLog` 对象表示为具有此功能访问权限的租户审计的事件。

AuditLog 对象的结构如下：

[inline-code-attrs-start title = 'AuditLog 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

审计日志是不可变的。它也不能手动写入。FastComments.com 只能决定何时写入审计日志。不过，您可以通过此 API 读取它。

审计日志中的事件在两年后过期。

---