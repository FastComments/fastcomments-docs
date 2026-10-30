---
也可以透過 REST API 從您自己的程式碼管理投票，例如從後端為評論新增投票、關閉投票，或匯出投票結果：

- [Poll Structure](/guide-api.html#poll-structure) 描述投票物件及其限制。
- [GET](/guide-api.html#poll-get)、[PUT](/guide-api.html#poll-put)、[PATCH](/guide-api.html#poll-patch) 以及 [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` 讀取、取代、編輯與移除評論上的投票。
- [PollVote Structure](/guide-api.html#poll-vote-structure) 描述單一投票。
- [GET](/guide-api.html#poll-votes-get)、[POST](/guide-api.html#poll-vote-post) 以及 [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` 列出、投下與移除投票。

也可以在建立評論時同時建立投票，只需將 `poll` 傳遞給 `POST /api/v1/comments`。

API 使用與小工具相同的設定：必須啟用投票，且未登入使用者的投票會被拒絕，除非允許匿名投票。匿名投票的個別投票無法透過 API 讀取。

---