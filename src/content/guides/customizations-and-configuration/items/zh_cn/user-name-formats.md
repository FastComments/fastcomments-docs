默认情况下，FastComments 会显示用户输入的姓名，或通过 SSO 传递给我们的姓名。

但是，可能希望以不同的方式掩盖或显示用户的姓名。例如，如果用户的姓名是 Allen Rex，您可能只想显示 "Allen R."。

这可以在 Widget Customization UI 中无需编码完成，位于名为 `Commenter Name Format` 的设置下：

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; title='Change Name Format' app-screenshot-end]

可用的格式有：

- Capitalize (display example user as Example User)
- Last Initial (display Example User as Example U.)
- All Initials (display Example User as E. U.)
- Show "Anonymous"

更改此设置会立即生效。用户在评论区域顶部（对他们自己）仍然会看到完整的用户名，但他们的评论将显示修改后的用户名。

用户名会在服务器端被掩码以保护用户。