## 觸發條件

觸發條件會在 FastComments 中發生某些事情時啟動 Zap。這三個觸發條件都是即時的：事件一發生，FastComments 就會透過 webhook 將事件傳送給 Zapier。不會輪詢您的帳戶，也不會在等待時消耗任何 API 額度。

| 觸發條件 | 觸發時機 |
|---------|-----------|
| New Comment | 有留言被發佈時。預設僅已核准、非垃圾留言會觸發。 |
| Updated Comment | 留言被編輯、核准、投票、置頂、鎖定或有其他變更時。 |
| Deleted Comment | 有留言被刪除時。 |

每個觸發條件都會傳回完整留言：id、頁面 URL 與 URL ID、留言者姓名與電子郵件、以 markdown 與 HTML 格式呈現的留言內容、投票數、核准與垃圾留言標記、語系設定、網域，以及任何提及。這些欄位與 Webhooks、Data Structures 文件中記載的 webhook 負載一致。

## 選項

**網域。** 每個觸發條件都有選用的網域篩選器，列出您帳戶上設定的網域。留空即可接收來自所有網域的事件。

**包含未核准與垃圾留言。** 僅適用於 New Comment 觸發條件。被留待審核或標記為垃圾的留言預設會被略過。當此類留言稍後被核准時，Updated Comment 觸發條件會為其觸發，因此若要對每則變為可見的留言做出反應的 Zap，應使用 Updated Comment 並對 approved 欄位設定篩選條件。

## 傳送機制說明

開啟 Zap 會在您的帳戶上建立一個 webhook 訂閱，可在 Webhooks 頁面上看到，來源為 **API**。關閉 Zap 則會移除該訂閱。Zapier 自身的限制適用於其每分鐘接受的事件數量；FastComments 會以遞增延遲重試失敗的傳送，並在訂閱持續失敗六天後將其停用。已停用的訂閱可以從 Webhooks 頁面重新啟用，或只需關閉再開啟 Zap 即可建立新的訂閱。

一個帳戶最多可容納 50 個 API 訂閱。每個使用 FastComments 觸發條件的 Zap 會佔用一個。