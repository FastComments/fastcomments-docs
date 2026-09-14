---
如果您的 val 已經知道訪客是誰，Secure SSO 會將該身份交給小工具，讓他們永遠不會看到第二次登入。無需建立任何端點，也不需要在執行時呼叫任何東西：您在伺服器端計算三個值，並將它們傳入小工具設定中。

Val Town 內建零設定登入功能，使用 `std/oauth`，因此訪客可以使用他們已有的 Val Town 帳號登入。您可以將其替換為您應用程式使用的任何方式；FastComments 部分則保持不變。

## 在伺服器上建立 Payload

API 密鑰會對 payload 進行簽名，且絕不能出現在瀏覽器程式碼中。從 npm 安裝 SDK，即可直接在 Val Town 的 Deno 執行環境中使用：

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

`getPayload()` 會回傳 `{ userDataJSONBase64, verificationHash, timestamp }`。這三個值就是傳遞到瀏覽器的全部內容。密鑰會對它們簽名，之後即被丟棄，因此頁面中沒有任何東西可以讓讀者偽造其他使用者。

## 將其傳入小工具

[inline-code-attrs-start title = '含 SSO 的小工具設定'; type='javascript' inline-code-attrs-end]
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

`oauthMiddleware` 會為您新增 `GET /auth/login`、`GET /auth/callback` 與 `POST /auth/logout`。請注意，登出是 **POST**，而小工具會以 GET 方式導向 `logoutURL`，因此請將 `logoutURL` 指向您自行實作的、會送出 POST 的小路由。

當訪客已登出時，僅傳入包含 `loginURL` 的 `sso`。小工具隨即顯示登入提示，而非匿名留言框。

## 常見問題

`timestamp` 為 epoch **毫秒**，不能是未來的時間，也不能超過兩天前。請在伺服器端於計算雜湊的同一個請求中產生它。在瀏覽器端產生是典型的失敗情形：產生的值與雜湊時使用的值不同，導致所有留言皆被拒絕。

千萬不要從身分提供者設定 `isAdmin` 或 `isModerator`。使用 Val Town 帳號登入並不代表該使用者應該擔任站點的管理員或版主。

請參閱 [SSO guide](/guide-sso.html) 以取得完整欄位清單、群組限制的討論串以及徽章資訊。

---