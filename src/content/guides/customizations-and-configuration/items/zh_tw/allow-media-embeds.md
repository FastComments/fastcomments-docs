---
預設情況下，FastComments 不允許在評論中使用 iframe。當您啟用媒體嵌入時，留言者可以貼上來自受信任供應商（例如 YouTube、Vimeo、SoundCloud 與 Spotify）的嵌入程式碼（`<iframe>` 片段），該內容會在評論中內嵌顯示。

為了安全起見，這不是一個用於用戶端 widget 的設定旗標。這是一個伺服器端設定，在每則評論儲存時進行驗證，因此無法從頁面上啟用。只有指向內建受信任供應商清單的 iframe 才會被允許。其他所有的 iframe 都會被移除。

此設定可在 widget 自訂頁面上完成，無需撰寫程式碼：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; title='Allow Media Embeds' app-screenshot-end]

### 新增您自己的供應商

如果您想允許來自內建受信任清單中未列出的供應商的嵌入，請在同一頁面的「額外嵌入網域」欄位新增其主機名稱。這些主機名稱會被允許，作為內建供應商之外的補充。比對是精確比對，因此請包含完整主機名稱（例如 player.example.com）。未列出的任何項目仍會被封鎖。

純文字評論欄與所見即所得（WYSIWYG）編輯器都支援貼上嵌入。在 WYSIWYG 編輯器中，嵌入會以可移除的區塊方式插入。

---