---
預設情況下，FastComments 不允許在評論中使用 iframe。啟用媒體嵌入後，評論者可以貼上來自受信任提供者（如 YouTube、Vimeo、SoundCloud 和 Spotify）的嵌入代碼（`<iframe>` 片段），它將在評論中內嵌顯示。

出於安全考量，這不是客戶端小工具設定旗標。它是伺服器端設定，於每則評論儲存時驗證，因此無法從頁面上開啟。僅允許指向內建受信任提供者清單的 iframe。其他任何 iframe 都會被移除。

此操作無需撰寫程式碼，可在小工具自訂頁面完成：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; alt='在小工具自訂頁面中開啟媒體嵌入設定，允許評論者貼上受信任的 iframe 嵌入'; title='允許媒體嵌入' app-screenshot-end]

### 添加您自己的提供者

如果您想允許來自未列於內建受信任清單的提供者的嵌入，請在同一頁面的「Additional Embed Domains」欄位中加入其主機名稱。這些主機名稱會與內建提供者一起被允許。匹配採用完全相符的方式，因此請包含完整的主機名稱（例如，player.example.com）。未列出的任何主機皆會被封鎖。

純文字評論框與 WYSIWYG 編輯器皆支援貼上嵌入內容。在 WYSIWYG 編輯器中，嵌入會以可移除的區塊形式插入。

---