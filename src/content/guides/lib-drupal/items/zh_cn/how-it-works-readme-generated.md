当用户访问启用了 FastComments 字段的实体时：

1. FastComments 的 JavaScript 小部件从 CDN 加载。
2. 如果配置了 SSO，用户的 Drupal 身份会传递给 FastComments。
3. 一个 `<noscript>` 回退为没有 JavaScript 的用户提供服务器渲染的评论（仅限 Live Comments 和 Streaming Chat 模式）。