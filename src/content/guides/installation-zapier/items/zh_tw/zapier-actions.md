## 動作與搜尋

動作會在 FastComments 中建立資料；搜尋則會查找資料，以便後續步驟使用。每個動作都會呼叫 FastComments REST API，並消耗與您自行程式碼呼叫相同的 API 點數：每次呼叫消耗 1 點，除非另有說明。

## 建立評論

在頁面上發表評論。

| Field | Required | Notes |
|-------|----------|-------|
| Page URL ID | Yes | 評論小工具在頁面上使用的 URL ID，評論會依此分組。 |
| Page URL | Yes | 完整的頁面 URL，用於通知電子郵件。 |
| Comment | Yes | FastComments markdown 格式的評論內容。 |
| Commenter Name | Yes | 名稱在每個電子郵件下必須唯一，若使用相同名稱但不同電子郵件會失敗。 |
| Commenter Email | No | 若該電子郵件尚未存在，系統會為其建立使用者。 |
| User ID | No | 已存在的 SSO 使用者 ID。會優先於名稱與電子郵件。 |
| Parent Comment ID | No | 設定以發表回覆。 |
| Approved, Verified | No | 兩者預設為 true。未批准的評論會保持隱藏，直到被審核。 |
| Posted At | No | 預設為現在時間。 |
| Avatar URL, Page Title, Locale | No | 語系預設為 `en_us`。 |
| Show Live In Widget | No | 即時將評論推送給觀眾。耗費 2 點而非 1 點。 |
| Run Spam Check, Send Emails | No | 預設為關閉。 |

## 建立頁面

在任何評論出現之前建立頁面記錄，以便能夠列出與限制。需要提供 URL ID、標題、URL，並可選擇允許查看的 SSO 群組 ID。

## 建立 SSO 使用者

建立單一登入使用者。需要提供您自己的使用者 ID、使用者名稱與電子郵件，並可選擇顯示名稱、顯示標籤、頭像、網站、群組 ID，以及通知與隱私旗標。管理員角色無法透過 Zapier 授予。

## 建立動態貼文

從 HTML 內容在 FastComments 動態中建立貼文，可選擇提供標題、作者、標籤，以及一個連結預覽。

## 建立雜湊標籤

建立評論者可使用的雜湊標籤，可選擇提供其連結的 URL。

## 標記評論

將評論標記為需要審核。提供執行標記的使用者 ID，或留空以使用 Zapier 整合的身分標記。

## 搜尋

| Search | Input | Returns |
|--------|-------|---------|
| Find Comment | Comment ID | 該評論，或無結果。 |
| Find SSO User | Email | 該 SSO 使用者，或無結果。 |
| Find Page | URL ID | 該頁面，或無結果。 |

搜尋不到結果不會使 Zap 失敗。可在 Zapier 的 "find or create" 模式中將搜尋與建立結合，於缺少頁面或使用者時自動建立。