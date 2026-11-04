如果你的 val 已经知道访客的身份，Secure SSO 会将该身份传递给评论组件，这样访客就无需再次登录。无需构建任何端点，运行时也无需调用任何东西：你只需在服务端计算三个值，并将它们传入组件配置即可。

Val Town 通过 `std/oauth` 提供零配置登录，因此访客可以使用他们已有的 Val Town 账户登录。你可以将其替换为你应用所使用的任何登录方式；FastComments 这部分不会改变。

## 在服务端构建 payload

API 密钥用于对 payload 进行签名，绝不能出现在浏览器代码中。从 npm 安装 SDK，它可以直接在 Val Town 的 Deno 运行时上使用：

[inline-code-attrs-start title = 'sso.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
import { SecureSSOPayloadBuilder } from "npm:fastcomments-sdk/server";

export function buildSSOPayload(user) {
  // id 对于同一个人必须保持稳定，否则他们每次登录都会获得一个新的评论身份。
  const id = `vt-${user.id}`;

  return new SecureSSOPayloadBuilder(Deno.env.get("FASTCOMMENTS_API_SECRET"), {
    id,
    // email 是必填项，且必须唯一。
    email: user.email ?? `${id}@users.noreply.val.town`,
    // username 是必填项，且不能是邮箱地址。
    username: user.username ?? id,
    displayName: user.username ?? undefined,
    avatar: user.links.profileImageUrl ?? undefined,
  }).getPayload();
}
[inline-code-end]

`getPayload()` 返回 `{ userDataJSONBase64, verificationHash, timestamp }`。这三个值就是传递给浏览器的全部内容。密钥对它们进行签名后即被丢弃，因此页面中的任何内容都无法让阅读者伪造其他用户。

## 将其传递给组件

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

  // ...使用此配置渲染组件
});

export default oauthMiddleware(app.fetch);
[inline-code-end]

`oauthMiddleware` 会为你添加 `GET /auth/login`、`GET /auth/callback` 和 `POST /auth/logout`。请注意，登出是一个 **POST** 请求，而组件通过 GET 导航到 `logoutURL`，因此请将 `logoutURL` 指向你自己的一个小路由，由它来提交 POST 请求。

当访客未登录时，传入仅包含 `loginURL` 的 `sso`。此时组件会显示登录提示，而不是匿名评论框。

## 常见问题

`timestamp` 是以**毫秒**为单位的 epoch 时间，不能是未来时间，也不能超过两天前。请在计算哈希的同一请求中于服务端生成它。在浏览器中生成它是典型的失败原因：该值与被哈希的值不一致，导致所有评论都被拒绝。

切勿根据身份提供商设置 `isAdmin` 或 `isModerator`。使用 Val Town 账户登录并不能说明谁应该管理你的网站。

完整的字段列表、基于群组的评论线程以及徽章，请参阅 [SSO 指南](/guide-customizations-and-configuration.html#sso-secure)。