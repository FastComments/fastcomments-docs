---
一个 `HashTag` 对象表示用户可以留下的标签。HashTags 可用于链接到外部内容或
将相关评论联系在一起。

The structure for the `HashTag` object is as follows:

[inline-code-attrs-start title = 'HashTag 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** 应以 "#" 或所需的字符开头。 **/
    tag: string
    /** 可选的 URL，话题标签可以指向它。UI 在点击时不会按话题标签过滤评论，而是重定向到此 URL。 **/
    url?: string
    /** 只读 **/
    createdAt: string
}
[inline-code-end]

Notes:

- In some API endpoints you will see that the hashtag is used in the URL. Remember to URI-Encoded values. For example, `#` should instead be represented as `%23`.
- Some of these fields are marked `READONLY` - these are returned by the API but cannot be set.
 
---