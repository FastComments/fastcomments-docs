Canvas 角色会在 LTI 启动期间自动映射到 FastComments 角色。无需手动配置。

#### 角色映射

| Canvas Role | FastComments Role | Permissions |
|---|---|---|
| **Administrator** | Admin | 完整的帐户访问权限，管理所有评论和设置 |
| **Instructor** | Moderator | 编辑和删除评论、置顶主题、管理讨论 |
| **Learner** | Commenter | 发布评论、回复、投票并使用提及功能 |

#### 工作原理

当用户从 Canvas 启动 FastComments 时，LTI 1.3 协议会包含他们的 Canvas 角色。FastComments 读取该角色并自动分配相应的权限。

如果用户具有多个角色（例如同时是 Instructor 和 Admin 的用户），将使用权限最高的角色。