允许用户向网站添加内容并在多种不同类型的设备上渲染这些内容时，安全涉及多个方面。

### 防止格式滥用

用户可能会通过滥用文本格式来编写故意在视觉上造成干扰的内容，从而降低讨论的价值。

FastComments 采取了一系列措施来防止与格式相关的滥用行为：

- 大量连续重复的换行会被折叠。
- 我们不渲染标题（它们会变成普通文本）。
- 我们不允许使用 CSS 或自定义颜色。

### 防止漏洞利用

在渲染 HTML 的系统中可能存在漏洞利用。FastComments 采取了多项措施来防止这种情况：

- 我们只允许明确定义的一组 HTML 标签。
- 我们只允许明确定义的一组 HTML 标签属性。
- 我们对所有输入进行净化和清理。
  - 这是通过 [DOMPurify](https://www.npmjs.com/package/dompurify) 和 [sanitizeHtml](https://www.npmjs.com/package/sanitize-html) 库实现的。
  - 我们选择这些库是因为它们经过了充分测试（每周下载量分别超过 400 万和 100 万次）。

这意味着用户无法执行诸如编写 `<script>` 或 `<style>` 标签，或尝试向图片或其他内容添加 `onload=alert()` 类型脚本之类的操作。

我们允许的 HTML 标签如下：

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

默认情况下不允许使用 `<iframe>` 标签。如果您开启了“允许媒体嵌入”，iframe 也将被允许，但仅当其来源是内置的可信提供商列表（如 YouTube、Vimeo、SoundCloud 和 Spotify）之一，或您明确添加的主机名时。来自任何其他来源的 iframe 都会被移除。