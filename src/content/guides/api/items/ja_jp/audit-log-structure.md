An `AuditLog` は、この機能へのアクセス権を持つテナント向けの監査イベントを表すオブジェクトです。

AuditLog オブジェクトの構造は以下の通りです:

[inline-code-attrs-start title = 'AuditLog の構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLog {
    id: string;
    /** イベントを実行したユーザー。 **/
    userId?: string;
    username?: string;
    resourceName: string;
    crudType: 'c' | 'r' | 'u' | 'd' | 'login';
    from: string;
    url?: string;
    ip?: string;
    /** イベントがブラウザから来た場合、そのブラウザ。 **/
    ua?: string;
    /** イベントが発生したセッションのハッシュ。個人の操作を関連付けるために使用します。セッション自体は含まれません。 **/
    sIdHashed?: string;
    when: string;
    description?: string;
    serverStartDate: string;
    /** イベントが実行されたオブジェクトの ID（実行者ではなく）。 **/
    targetId?: string;
    /** そのオブジェクトの人間が読めるラベル（例: "jsmith (jsmith@example.com)"）。 **/
    targetLabel?: string;
    objectDetails?: object;
}
[inline-code-end]

`targetId` と `targetLabel` はイベントが実行された対象を示し、`userId` と `username` は実行者を示します。更新の場合、`objectDetails.changes` は実際に変更された `{field: {from, to}}` マップを保持します。

監査ログは不変です。また、手動で書き込むことはできません。FastComments.com が監査ログへの書き込みタイミングを決定します。ただし、この API を通じて監査ログを読み取ることは可能です。

監査ログ内のイベントは2 年後に期限切れとなります。