在審核和檢視評論串時，希望能直接跳轉到某個串以在審核過程中取得上下文。

這表示使用者的流程會從 Comment Moderation 頁面開始，然後必須從個別評論前往
包含該評論的頁面，等該頁面載入，等評論載入，然後再捲動到該評論。

然而，FastComments 提供更快速的方法。在評論審核頁面中，每則評論旁邊的右下角都有一個 "檢視評論" 按鈕。

[app-screenshot-start url='/auth/my-account/moderate-comments?filter=&text-search=&page=1&count=1&demo=true'; linkUrl='/auth/my-account/moderate-comments'; selector = '.comments .comment-component'; title='A Comment' app-screenshot-end]

如果這則評論有回覆，按鈕文字會改為顯示回覆數量，但點擊它仍會執行相同的動作。

此按鈕會帶您到 **評論串檢視器**。

評論串檢視器是由 FastComments 提供並託管的一個小型、快速載入的應用程式，它會呈現該評論所在頁面的評論串，並自動捲動到該評論。

這讓審核者能快速取得所需的上下文，而不必等待另一個頁面載入。

---