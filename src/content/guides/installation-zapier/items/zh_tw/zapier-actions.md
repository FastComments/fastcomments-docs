## 動作與搜尋

動作會在 FastComments 中建立資料；搜尋則會查找資料，以便後續步驟使用。每個動作都會呼叫 FastComments REST API，並消耗與您自行程式碼呼叫相同的 API 點數：每次呼叫消耗 1 點，除非另有說明。

## 建立評論

在頁面上發表評論。

| 欄位 | 必填 | 備註 |
|------|------|------|
| Page URL ID | 是 | 評論小工具在頁面上使用的 URL ID。評論會依此分組。 |
| Page URL | 是 | 完整的頁面 URL，用於通知電子郵件。 |
| Comment | 是 | FastComments markdown 格式的評論內容。 |
| Commenter Name | 是 | 名稱在每個電子郵件下必須唯一，若使用不同的電子郵件重複名稱會失敗。 |
| Commenter Email | 否 | 若該電子郵件尚未存在，會為其建立使用者。 |
| User ID | 否 | 已存在的 SSO 使用者 ID。會優先於名稱與電子郵件使用。 |
| Parent Comment ID | 否 | 設定後即為回覆。 |
| Approved, Verified | 否 | 兩者預設為 true。未批准的評論會保持隱藏，直到被審核。 |
| Posted At | 否 | 預設為現在。 |
| Avatar URL, Page Title, Locale | 否 | Locale 預設為 `en_us`。 |
| Show Live In Widget | 否 | 即時將評論推送給觀眾。會消耗 2 點而非 1 點。 |
| Run Spam Check, Send Emails | 否 | 預設為關閉。 |

## 建立或更新頁面

在任何評論出現之前先建立頁面記錄，讓它可以被列出與受限。需要提供 URL ID、標題、URL，並可選擇允許檢視的 SSO 群組 ID。若已存在相同 URL ID 的頁面，會以提供的欄位更新該頁面，讓 Zap 能對同一頁面重複執行。

## 建立或更新 SSO 使用者

建立單一登入（SSO）使用者。需要您自己的使用者 ID、使用者名稱與電子郵件，並可選擇顯示名稱、顯示標籤、頭像、網站、群組 ID，以及通知與隱私旗標。若該 ID 已存在，則改為更新。管理員角色無法透過 Zapier 授予。

## 建立 Feed 文章

從 HTML 內容在 FastComments Feed 中建立文章。必須提供作者使用者 ID（FastComments 或 SSO 使用者 ID）；標題、標籤與一個連結預覽為可選項目。

## 建立或更新雜湊標籤

建立評論者可使用的雜湊標籤，並可選擇其連結的 URL。若標籤已存在，則改為更新。

## 標記評論

將評論標記為需要審核。需要提供執行標記的使用者 ID；可使用「建立評論」回傳的作者 ID。

## 搜尋

| 搜尋 | 輸入 | 回傳 |
|------|------|------|
| Find Comment | Comment ID | 該評論，若無則回傳空。 |
| Find SSO User | Email | 該 SSO 使用者，若無則回傳空。 |
| Find Page | URL ID | 該頁面，若無則回傳空。 |

找不到結果的搜尋不會使 Zap 失敗。Find SSO User 與 Find Page 提供 Zapier 的「若不存在則建立」選項，當找不到時會執行相對應的建立動作。