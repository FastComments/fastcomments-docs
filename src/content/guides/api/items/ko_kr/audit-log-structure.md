`AuditLog`은 이 기능에 접근 권한이 있는 테넌트의 감사 이벤트를 나타내는 객체입니다.

AuditLog 객체의 구조는 다음과 같습니다:

[inline-code-attrs-start title = 'AuditLog 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string;
    /** 이벤트를 수행한 사람. **/
    userId?: string;
    username?: string;
    resourceName: string;
    crudType: 'c' | 'r' | 'u' | 'd' | 'login';
    from: string;
    url?: string;
    ip?: string;
    /** 이벤트를 수행한 브라우저(해당되는 경우). **/
    ua?: string;
    /** 이벤트가 발생한 세션의 해시값으로, 한 사람의 행동을 연관시키기 위해 사용됩니다. 세션 자체는 포함되지 않습니다. **/
    sIdHashed?: string;
    when: string;
    description?: string;
    serverStartDate: string;
    /** 이벤트가 수행된 객체의 ID(누가 수행했는지와는 별도). **/
    targetId?: string;
    /** 해당 객체에 대한 사람이 읽을 수 있는 라벨, 예: "jsmith (jsmith@example.com)". **/
    targetLabel?: string;
    objectDetails?: object;
}
[inline-code-end]

`targetId`와 `targetLabel`은 이벤트가 수행된 대상을 설명하고; `userId`와 `username`은 누가 수행했는지를 설명합니다. 업데이트의 경우, `objectDetails.changes`는 실제로 변경된 내용을 `{field: {from, to}}` 형태의 맵으로 보유합니다.

감사 로그는 불변이며, 수동으로 기록할 수 없습니다. FastComments.com만이 감사 로그에 언제 기록할지를 결정할 수 있습니다. 그러나 이 API를 통해 로그를 읽을 수 있습니다.

감사 로그의 이벤트는 2년 후에 만료됩니다.