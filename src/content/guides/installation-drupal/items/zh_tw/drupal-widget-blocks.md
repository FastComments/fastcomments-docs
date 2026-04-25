此模組提供數個區塊，可以從 `Structure > Block layout` (`/admin/structure/block`) 放置。

- **FastComments Widget** - 主要的留言元件。會自動偵測當前實體。它會跳過已附加 FastComments 欄位的實體，因此您不會在相同頁面看到重複的元件。
- **FastComments Live Chat** - 即時串流聊天。可以與相同頁面的評論欄位並排放置。
- **FastComments Collab Chat** - 文字選取的註解與討論。
- **FastComments Image Chat** - 基於座標的圖片註解。訪客點擊圖片即可留下與特定位置相關聯的評論。
- **FastComments Recent Comments** - 顯示整個網站的最新評論。數量可在區塊上設定。
- **FastComments Top Pages** - 顯示網站上評論數最多的頁面。

以內容為中心的區塊（Live Chat、Collab Chat、Image Chat）會自動偵測當前實體，並在非實體頁面時退回使用基於路徑的識別符。這表示它們可在分類法頁面、Views 與自訂路由上運作，無需任何額外設定。