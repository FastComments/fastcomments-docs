当允许用户向网站添加内容并在多种设备上呈现这些内容时，安全性涉及多个方面。

### Preventing Formatting Abuse

有人可能通过滥用文本格式，故意编写在视觉上分散注意力并削弱讨论价值的内容。

FastComments 在防止格式滥用方面采取了若干措施：

- 大量连续重复的新行会被折叠。
- 我们不呈现标题（它们会变为普通文本）。
- 我们不允许 CSS 或自定义颜色。

### Preventing Exploits

在呈现 HTML 的系统中可能出现漏洞利用。FastComments 采取了几项措施来防止这类情况：

- 我们仅允许一组明确定义的 HTML 标签。
- 我们仅允许一组明确定义的 HTML 标签属性。
- 我们对所有输入进行净化和清理。
  - 这是通过 [DOMPurify](https://www.npmjs.com/package/dompurify) 和 [sanitizeHtml](https://www.npmjs.com/package/sanitize-html) 库完成的。
  - 我们选择这些库是因为它们经过充分测试（每周下载量分别超过 400 万和 100 万）。

这意味着用户不能做诸如编写 `<script>` 或 `<style>` 标签，或尝试在图片或其他内容中添加 `onload=alert()` 类型的脚本。

The HTML tags we allow are as follows:

- `<b>`
- `<em>`
- `<u>`
- `<i>`
- `<strike>`
- `<pre>`
- `<span>`
- `<code>`
- `<img>`
- `<a>`
- `<strong>`
- `<ul>`
- `<ol>`
- `<li>`
- `<br>`

The `<iframe>` tag is not allowed by default. If you turn on Allow Media Embeds, iframes are also permitted, but only when their source is one of a built-in list of trusted providers (such as YouTube, Vimeo, SoundCloud, and Spotify) or a hostname you have explicitly added. Iframes from any other source are removed.