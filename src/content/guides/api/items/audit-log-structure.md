An `AuditLog` is an object that represents an audited event for tenants that have access to this feature.

The structure for the AuditLog object is as follows:

[inline-code-attrs-start title = 'AuditLog Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

The audit log is immutable. It also cannot be written to manually. FastComments.com may only decide when to write to the audit log. However, you may read from it via this API.

Events in the audit log expire after two years.
