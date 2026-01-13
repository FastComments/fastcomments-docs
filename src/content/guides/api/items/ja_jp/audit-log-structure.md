`AuditLog` はこの機能にアクセスできるテナントの監査イベントを表すオブジェクトです。

AuditLog オブジェクトの構造は次のとおりです:

[inline-code-attrs-start title = 'AuditLog の構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

監査ログは不変です。手動で書き込むこともできません。FastComments.com のみが監査ログに書き込むタイミングを決定できます。ただし、この API を通じて読み取ることはできます。

監査ログ内のイベントは2年で期限切れになります。