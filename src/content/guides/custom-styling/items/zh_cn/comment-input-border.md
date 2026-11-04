评论框周围的边框由文本区域自身的边框加上小组件在其周围绘制的几条细线组成。要更改其颜色或圆角，请设置这些 CSS 变量，而不是直接对 `textarea` 进行样式设置。这些变量会同时重新设置边框的每个部分，因此圆角和颜色始终保持一致。

| 变量 | 作用 | 默认值 |
|---|---|---|
| `--fc-input-border-color` | 边框颜色 | `#bfbfbf` |
| `--fc-input-border-color-focus` | 用户输入时的边框颜色 | `#555` |
| `--fc-input-border-radius` | 圆角的圆度 | `11px` |
| `--fc-input-border-start-start-radius` | 方形左上角的圆度（在从右到左的语言中为右上角） | `0` |

将 CSS 添加到[小组件自定义页面](https://fastcomments.com/auth/my-account/customize-widget)上的 **Custom CSS** 框中，或通过 `customCSS` 选项传入。您只需设置想要更改的变量。

您还可以使用 Custom CSS 框正下方的 **Comment box border** 辅助工具，它会为您自动生成这些 CSS。

## 更改边框颜色

[inline-code-attrs-start title = '边框颜色'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## 输入时更改边框颜色

[inline-code-attrs-start title = '输入时的边框颜色'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## 将四个角全部设为圆角

默认情况下左上角是方形的。将两个圆角变量都设置相同的值即可将四个角全部设为圆角：

[inline-code-attrs-start title = '将四个角全部设为圆角'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## 方形边角

[inline-code-attrs-start title = '方形边角'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## 匹配您的品牌

[inline-code-attrs-start title = '品牌颜色和边角'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## 深色模式下的不同颜色

当小组件处于深色模式时，它会带有 `dark` 类，因此您可以为深色模式设置不同的值：

[inline-code-attrs-start title = '深色模式边框颜色'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## 为什么不直接对文本区域进行样式设置？

小组件会在文本区域周围自行绘制评论框边框的一部分。如果您只在 `textarea` 上设置 `border-color` 或 `border-radius`，这些线条将保持默认样式，边框看起来会不协调，例如一条方形线条穿过圆角。上述变量会同时更改两者。