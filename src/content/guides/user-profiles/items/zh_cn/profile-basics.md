FastComments 中的用户资料为每个用户提供一个专用空间，以展示他们在您的社区中的身份、活动和贡献。

### 什么是用户资料？

用户资料是为每个 FastComments 用户定制的页面，显示：

- **Profile Header** - 可自定义的背景图片，用于个性化资料页
- **Avatar** - 用户的头像及在线/离线状态指示
- **Display Information** - 用户名、显示名称和可选的国家旗帜
- **Bio** - 个人简介或介绍
- **Social Links** - 用户社交媒体资料和网站的链接
- **Badges** - 获得的成就与荣誉
- **Statistics** - 用户声望（karma）和评论总数
- **Communities** - 用户活跃的站点/域名

### 访问用户资料

有几种方式可以访问用户的资料：

1. **Click on an avatar** - 在评论组件中，点击任何用户的头像以查看其资料
2. **Click on a username** - 评论中的用户名是可点击的资料链接
3. **Direct URL** - 访问 `https://fastcomments.com/auth/user-profile/[userId]`

### 资料视图

查看资料时，会根据是查看自己的资料还是他人的资料显示不同的标签页：

#### 您自己的资料
- **Notifications** - 您的通知和提及
- **Recent Activity** - 您在所有社区的评论历史
- **Profile Comments** - 其他人在您的资料页留下的评论
- **Direct Messages** - 与其他用户的私人对话

#### 他人的资料
- **Recent Activity** - 他们的公开评论历史（如果未设置为私密）
- **Profile Comments** - 其资料页上的评论（如果未设置为私密）
- **Direct Messages** - 发起或继续私人对话（如果他们允许私信）

### 在线状态

用户资料显示实时在线状态：
- **Green indicator** - 用户当前在线
- **No indicator** - 用户离线

这可以帮助您知道某人在何时积极使用平台，尤其在发送私信时非常有用。

### 用户类型

FastComments 支持两类拥有资料的用户：

1. **Regular Users** - 直接在 FastComments 注册的用户
2. **SSO Users** - 通过您站点的单点登录（SSO）集成进行身份验证的用户

两类用户都可以使用完整的资料系统，尽管根据您的 SSO 配置，SSO 用户在编辑某些字段（例如头像）时可能会有一些限制。