`tenantId: "demo"` 是一个共享的公共沙盒。它无需注册即可使用，这也是示例使用它的原因，但其他使用 FastComments 的人会写入相同的线程，任何人都可以对其进行审核。在发布任何重要内容之前请切换。

您的租户 ID 位于 [API secret page](https://fastcomments.com/auth/my-account/api-secret)。

租户 ID 是公开的，应该放在浏览器代码中。API 密钥不是公开的，且本页的内容不需要它。

## 从环境变量读取

Val Town 的 vals 在免费层是公开的，因此它们的源代码是全局可读的。请将任何敏感信息保存在环境变量中，并使用 `Deno.env.get` 读取：

[inline-code-attrs-start title = '配置.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
export const TENANT_ID = Deno.env.get("FASTCOMMENTS_TENANT_ID") ?? "demo";

// 仅在 eu.fastcomments.com 创建的账户会将此设置为 "eu".
export const REGION = Deno.env.get("FASTCOMMENTS_REGION") ?? "";

export const CDN = REGION === "eu"
  ? "https://cdn-eu.fastcomments.com"
  : "https://cdn.fastcomments.com";
[inline-code-end]

这在 Val Town 上尤为重要，原因有二：**重新混合一个 val 会复制环境变量的键，但不会复制其值。** 保存在环境变量中的密钥不会随你的 val 进入他人的账户，而写入文件的密钥会。

回退到 `"demo"` 可以让在设置自己租户之前重新混合该 val 的任何人都能正常使用。

## EU 账户

一个账户、其数据以及密钥都位于同一地区。如果您的账户是在 `eu.fastcomments.com` 创建的，则每个小部件配置也需要 `region: "eu"`，并且脚本会从 `cdn-eu.fastcomments.com` 加载。否则保持默认即可。

---