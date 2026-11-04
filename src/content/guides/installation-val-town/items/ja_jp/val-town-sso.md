すでに訪問者が誰であるかを val が把握している場合、Secure SSO はその ID をウィジェットに渡すため、訪問者は2回目のログインを目にすることはありません。構築すべきエンドポイントも、実行時に呼び出すべき API もありません。3つの値をサーバー側で計算し、ウィジェットの設定に渡すだけです。

Val Town は `std/oauth` による設定不要のログインを提供しているため、訪問者は既に持っている Val Town アカウントでサインインできます。これをお使いのアプリで使用している認証方式に差し替えても、FastComments 側は変更不要です。

## サーバーでペイロードを構築する

API シークレットはペイロードに署名するためのもので、ブラウザーコードに到達してはいけません。npm から SDK をインストールしてください。Val Town の Deno ランタイムでそのまま動作します:

[inline-code-attrs-start title = 'sso.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { SecureSSOPayloadBuilder } from "npm:fastcomments-sdk/server";

export function buildSSOPayload(user) {
  // id must be stable for the same person, or they get a new comment identity on every login.
  const id = `vt-${user.id}`;

  return new SecureSSOPayloadBuilder(Deno.env.get("FASTCOMMENTS_API_SECRET"), {
    id,
    // email is required and must be unique.
    email: user.email ?? `${id}@users.noreply.val.town`,
    // username is required and cannot be an email.
    username: user.username ?? id,
    displayName: user.username ?? undefined,
    avatar: user.links.profileImageUrl ?? undefined,
  }).getPayload();
}
[inline-code-end]

`getPayload()` は `{ userDataJSONBase64, verificationHash, timestamp }` を返します。ブラウザーに到達するのはこの3つの値だけです。シークレットはこれらに署名した後で破棄されるため、ページ内のどの情報を使っても別のユーザーを偽装することはできません。

## ウィジェットに渡す

[inline-code-attrs-start title = 'Widget config with SSO'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { getOAuthUserData, oauthMiddleware } from "https://esm.town/v/std/oauth/middleware.ts";

app.get("/", async (c) => {
  const session = await getOAuthUserData(c.req.raw);
  const user = session?.user;

  const config = {
    tenantId: TENANT_ID,
    urlId: "my-thread",
    ...(user
      ? { sso: { ...buildSSOPayload(user), logoutURL: "/logout" } }
      : { sso: { loginURL: "/auth/login" } }),
  };

  // ...render the widget with this config
});

export default oauthMiddleware(app.fetch);
[inline-code-end]

`oauthMiddleware` は `GET /auth/login`、`GET /auth/callback`、`POST /auth/logout` を自動的に追加します。なお、ログアウトは **POST** ですが、ウィジェットは `logoutURL` に GET で遷移するため、`logoutURL` には POST を送信する小さな独自ルートを指定してください。

訪問者がログアウトしている場合は、`loginURL` のみを含む `sso` を渡します。するとウィジェットは匿名コメントボックスの代わりにログインプロンプトを表示します。

## よくある問題

`timestamp` はエポック**ミリ秒**であり、未来の時刻であってはならず、2日以上古くてもいけません。ハッシュを計算するのと同じリクエスト内でサーバー側で生成してください。ブラウザー側で生成するのが典型的な失敗パターンです。その場合、値がハッシュ化されたものと異なり、すべてのコメントが拒否されます。

ID プロバイダーから `isAdmin` や `isModerator` を設定してはいけません。Val Town アカウントでのサインインは、誰があなたのサイトをモデレートすべきかを示すものではありません。

フィールドの一覧、グループ制限付きスレッド、バッジの詳細については、[SSO ガイド](/guide-customizations-and-configuration.html#sso-secure) を参照してください。