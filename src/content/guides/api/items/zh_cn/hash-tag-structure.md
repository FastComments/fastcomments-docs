`HashTag` 对象表示用户可以留下的标签。HashTags 可用于链接到外部内容，或将相关评论关联在一起。

`HashTag` 对象的结构如下：

[inline-code-attrs-start title = 'HashTag 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** 应以 "#" 或所需字符开头。 **/
    tag: string
    /** 一个可选的 URL，hashtag 可以指向它。点击时，UI 将重定向到该 URL，而不是按 hashtag 筛选评论。 **/
    url?: string
    /** 只读 **/
    createdAt: string
}
[inline-code-end]

注意事项：

- 在某些 API 端点中，你会看到 hashtag 被用于 URL 中。请记住对值进行 URI 编码。例如，`#` 应表示为 `%23`。
- 其中一些字段被标记为 `READONLY`（只读）——它们由 API 返回，但无法设置。