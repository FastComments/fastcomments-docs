FastComments 用户徽章由具有 `Customize Data` 权限的管理员配置。

This is done via [Customize -> Badges](https://fastcomments.com/auth/my-account/configure-badges) in your admin dashboard。

当用户被授予徽章时，该徽章会显示在他们的个人资料和评论上。

When adding a badge we can setup a `Display Label`, which is the name the user sees associated with the badge. For example, if we add a `Comment Count` badge
我们可能不想显示那个技术性名称，因为听起来很乏味。我们可能会称之为 `Super Member` 或类似名字。徽章也可以叠加或相互替换，正如我们将在
本文档后面所述。

徽章还具有可配置的阈值。

徽章可以创建，之后通过取消选中 `Enabled` 将其禁用。禁用徽章意味着它将不再被自动授予，也不会在 Award Manual Badge 菜单中显示，但用户会保留该徽章。

### Badge Display Types

徽章可以是图片或文本徽章，支持一些基本样式（文本颜色、背景颜色和边框颜色）。您也可以通过 CSS 对徽章进行样式设置。

图片徽章可以是 GIF 图像以显示动画徽章。

### Tip - Do Not Remove Badges!

用户喜爱徽章。他们往往非常在意徽章，即使某个徽章是你无意中添加的错误，你想要更改徽章图标，用户也会在意。

If we've learned anything, it's extremely difficult to take something away from users. Removing a badge because you as an owner of the site no
longer like it, or want to make changes, may result in a very angry crowd of users that suddenly leave your site out of frustration. For this reason
`Delete` was not even an option for the first few months we released this feature - however we ended up having to add it. But please, use delete with caution. We have
seen many long time, multi-year, users get very frustrated and leave their communities because administrators decided to delete a badge.

如果你必须停止使用某个徽章，建议你仅将其禁用，以便用户保留他们的徽章。

### Badge Reprocessing

当徽章被添加或更改时，系统将回溯检查与您站点互动的用户，以确定他们是否应获得该徽章。这将在管理仪表板的徽章页面中可见，因为在显示拥有该徽章的用户数量的位置会显示一个加载指示器（spinner）。这是因为用户数量正在被确定。

### Seeing Who Has a Badge

在徽章列表中，每个链接都有一个 `View Users` 选项，用于显示已获得或被手动授予该徽章的用户列表。