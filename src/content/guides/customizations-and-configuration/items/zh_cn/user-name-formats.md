---
默认情况下，FastComments 会显示用户输入的名称，或通过 SSO 传递给我们的名称。

然而，可能需要以不同的方式掩码或显示用户的名称。例如，如果用户的名称是 Allen Rex，您可能只想显示 “Allen R.”。

这可以在 Widget Customization UI 中无需编写代码完成，位于名为 `Commenter Name Format` 的设置下：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; alt='Commenter Name Format 下拉菜单打开，包含如 Capitalize、Last Initial 和 All Initials 等选项'; title='更改名称格式' app-screenshot-end]

可用的格式有：

- Capitalize（将示例用户显示为 Example User）
- Last Initial（将 Example User 显示为 Example U.）
- All Initials（将 Example User 显示为 E. U.）
- 显示 “Anonymous”

更改此设置的效果是即时的。用户仍会在评论区域顶部看到自己的完整用户名，但他们的评论将显示修改后的用户名。

用户名在服务器端被掩码，以保护用户。
---