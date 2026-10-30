投票也可以通过 REST API 从您自己的代码进行管理，例如从后端向评论添加投票、关闭投票或导出投票结果：

- [Poll Structure](/guide-api.html#poll-structure) 描述投票对象及其限制。
- [GET](/guide-api.html#poll-get)、[PUT](/guide-api.html#poll-put)、[PATCH](/guide-api.html#poll-patch) 和 [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` 读取、替换、编辑和删除评论上的投票。
- [PollVote Structure](/guide-api.html#poll-vote-structure) 描述单个投票。
- [GET](/guide-api.html#poll-votes-get)、[POST](/guide-api.html#poll-vote-post) 和 [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` 列出、投票和删除投票。

也可以通过将 `poll` 传递给 `POST /api/v1/comments` 来在创建评论的同时创建投票。

API 的设置与小部件相同：必须启用投票，且未登录用户的投票将被拒绝，除非允许匿名投票。匿名投票的单个投票无法通过 API 读取。