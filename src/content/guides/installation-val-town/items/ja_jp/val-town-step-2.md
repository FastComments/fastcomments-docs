`tenantId: "demo"` は共有のパブリックサンドボックスです。サインアップ不要で動作するため、例で使用されていますが、FastComments を試す他のすべてのユーザーは同じスレッドに書き込み、誰でもそれらをモデレートできます。重要なものを公開する前に切り替えてください。

テナント ID は [API シークレットページ](https://fastcomments.com/auth/my-account/api-secret) にあります。

テナント ID は公開情報で、ブラウザコードに含めるべきものです。API シークレットはそうではなく、このページの何もそれを必要としません。

## 環境変数から読み取る

Val Town の val は無料プランで公開されているため、ソースは全世界から読み取れます。機密情報は環境変数に保存し、`Deno.env.get` で読み取ります。

[inline-code-attrs-start title = 'config.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
export const TENANT_ID = Deno.env.get("FASTCOMMENTS_TENANT_ID") ?? "demo";

// Only accounts created on eu.fastcomments.com set this, to "eu".
export const REGION = Deno.env.get("FASTCOMMENTS_REGION") ?? "";

export const CDN = REGION === "eu"
  ? "https://cdn-eu.fastcomments.com"
  : "https://cdn.fastcomments.com";
[inline-code-end]

これは Val Town で通常より重要です。理由は二つ目に、**val をリミックスすると環境変数のキーはコピーされますが、値はコピーされません。** 環境変数に保存されたシークレットは、あなたの val が他人のアカウントに移行しても引き継がれません。ファイルに書かれたシークレットは引き継がれます。

`"demo"` にフォールバックすることで、独自のテナントを設定する前にリミックスした人でも val が動作し続けます。

## EU アカウント

アカウント、そのデータ、キーはすべて同一リージョンに存在します。もし `eu.fastcomments.com` で作成した場合、すべてのウィジェット設定にも `region: "eu"` が必要で、スクリプトは `cdn-eu.fastcomments.com` からロードされます。そうでなければ、両方ともそのままにしてください。

---