FastComments 访问控制通过将 `Pages` 和 `Users` 分配到所需的组来工作。

组只是一个字符串标识符，例如 `GREEN` 或 `abc-123`。

`Users` 和 `Pages` 不仅限于一个组。它们分别限制为 `100` 和 `1000` 个组。 

#### 访问未授权页面

如果用户尝试访问他们无权访问的页面，将看到如下错误消息：

<div class="screenshot white-bg">
    <div class="title">授权失败示例</div>
    <img class="screenshot-image" src="/images/sso-unauthorized-message.png" alt="Authorization Failure Example" />
</div>

该消息文本可自定义。