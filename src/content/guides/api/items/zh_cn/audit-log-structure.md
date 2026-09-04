An `AuditLog` 是一个对象，表示对拥有此功能访问权限的租户的审计事件。

AuditLog 对象的结构如下：

[inline-code-attrs-start title = '审计日志结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string;
    /** 执行此事件的用户。 **/
    userId?: string;
    username?: string;
    resourceName: string;
    crudType: 'c' | 'r' | 'u' | 'd' | 'login';
    from: string;
    url?: string;
    ip?: string;
    /** 执行此事件的浏览器（如果来自浏览器）。 **/
    ua?: string;
    /** 事件来源会话的哈希，用于关联同一人的操作。永远不包含会话本身。 **/
    sIdHashed?: string;
    when: string;
    description?: string;
    serverStartDate: string;
    /** 事件执行对象的 ID（区别于执行者）。 **/
    targetId?: string;
    /** 该对象的可读标签，例如 "jsmith (jsmith@example.com)"。 **/
    targetLabel?: string;
    objectDetails?: object;
}
[inline-code-end]

`targetId` 和 `targetLabel` 描述事件作用的对象；`userId` 和 `username` 描述执行者。对于更新操作，`objectDetails.changes` 保存一个 `{field: {from, to}}` 映射，记录实际的更改内容。

审计日志是不可变的，且不能手动写入。FastComments.com 只能决定何时写入审计日志。不过，您可以通过此 API 读取审计日志。

审计日志中的事件将在两年后过期。