An `AuditLog` is an object that represents an audited event for tenants that have access to this feature.

The structure for the AuditLog object is as follows:

[inline-code-attrs-start title = 'AuditLog Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

`targetId` and `targetLabel` describe what the event was performed on; `userId` and `username` describe who performed it. For updates, `objectDetails.changes` holds a `{field: {from, to}}` map of what actually changed.

The audit log is immutable. It also cannot be written to manually. FastComments.com may only decide when to write to the audit log. However, you may read from it via this API.

Events in the audit log expire after two years.
