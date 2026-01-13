---
`AuditLog`는 이 기능에 접근 권한이 있는 테넌트에 대해 감사된 이벤트를 나타내는 객체입니다.

AuditLog 객체의 구조는 다음과 같습니다:

[inline-code-attrs-start title = 'AuditLog 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

감사 로그는 불변입니다. 수동으로 쓸 수도 없습니다. FastComments.com만 감사 로그에 기록할 시점을 결정할 수 있습니다. 그러나 이 API를 통해 감사 로그를 읽을 수 있습니다.

이 감사 로그의 이벤트는 2년 후에 만료됩니다.

---