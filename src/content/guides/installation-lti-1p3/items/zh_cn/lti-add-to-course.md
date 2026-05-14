一旦 FastComments 在你的 LMS 中注册，教师将以添加任何其他 LTI 外部工具的相同方式将其添加到课程中。

#### D2L Brightspace

在课程的内容区：

1. 点击 **Add Existing Activities** > **External Learning Tools**。
2. 从列表中选择 **FastComments**。
3. 该工具会作为内容区的一个主题出现。打开一次以为该资源初始化评论线程。

#### Moodle

在课程中：

1. 打开 **Edit mode**。
2. 在想要添加评论的章节，点击 **Add an activity or resource**。
3. 在活动选择器中选择 **FastComments**。
4. 保存。学生会在该章节中看到嵌入的评论线程。

#### Blackboard Learn

在课程中：

1. 导航到一个 Content Area。
2. 点击 **Build Content** > **FastComments**（位于 “Learning Tools” 下）。
3. 配置名称并提交。

#### Sakai

站点维护者通过 **Site Info** > **Manage Tools** > 向下滚动到 **External Tools** > 选择 **FastComments** > **Continue** 来添加该工具。

#### How Threads Are Scoped

FastComments 为每个 **(LMS instance, course, resource link)** 创建一个单独的评论线程。这意味着：

- 即使使用相同的工具名称，同一 LMS 中的两个不同课程也会得到独立的线程。
- 在同一课程中两处使用相同的 FastComments 工具会创建两个线程。
- 在同一 FastComments 帐户下的两个不同 Brightspace 安装会获得不同的线程 — LMS 主机名是线程标识符的一部分。

#### SSO

学生不会看到登录界面。LMS 通过 LTI 启动将他们的身份（姓名、电子邮件、头像、角色）发送到 FastComments，FastComments 会自动为他们登录。他们的评论会归属于他们的 LMS 帐户。

具有 LMS 角色 **Instructor** 或 **Administrator** 的用户会自动映射为该线程的 FastComments 管理员/版主角色。