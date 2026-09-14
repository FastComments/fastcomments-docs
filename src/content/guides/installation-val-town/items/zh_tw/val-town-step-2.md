`tenantId: "demo"` 是一個共享的公共沙盒。它無需註冊即可運作，這也是範例使用它的原因，但其他所有使用 FastComments 的人都會寫入相同的討論串，且任何人都可以對其進行審核。請在發布任何重要內容之前切換。

您的租戶 ID 位於 [API 密鑰頁面](https://fastcomments.com/auth/my-account/api-secret)。

租戶 ID 是公開的，應放在瀏覽器程式碼中。API 密鑰則不應如此，且本頁面不需要任何 API 密鑰。

## 從環境變數讀取

Val Town 的 vals 在免費層是公開的，因此它們的來源是全世界可讀的。請將任何敏感資訊放在環境變數中，使用 `Deno.env.get` 讀取：

[inline-code-attrs-start title = 'config.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
export const TENANT_ID = Deno.env.get("FASTCOMMENTS_TENANT_ID") ?? "demo";

// Only accounts created on eu.fastcomments.com set this, to "eu".
export const REGION = Deno.env.get("FASTCOMMENTS_REGION") ?? "";

export const CDN = REGION === "eu"
  ? "https://cdn-eu.fastcomments.com"
  : "https://cdn.fastcomments.com";
[inline-code-end]

這在 Val Town 上尤為重要，原因有二：**重新混合一個 val 會複製環境變數的鍵名，但不會複製其值。** 放在環境變數中的密鑰不會隨你的 val 進入他人的帳戶。寫入檔案的密鑰則會。

回退到 `"demo"` 可讓該 val 在任何人在設定自己的租戶之前重新混合時仍能正常運作。

## EU 帳戶

一個帳戶、其資料與金鑰都位於同一區域。如果您的帳戶是在 `eu.fastcomments.com` 建立的，則每個小工具設定也需要 `region: "eu"`，且腳本會從 `cdn-eu.fastcomments.com` 載入。否則請保持兩者不變。

---