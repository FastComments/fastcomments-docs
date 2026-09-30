The border around the comment box is made of the text area's own border plus a few thin lines the widget draws around it. To change its color or corner roundness, set these CSS variables instead of styling the `textarea` directly. The variables restyle every part of the border at once, so the corners and colors always line up.

댓글 상자 주변의 테두리는 텍스트 영역 자체의 테두리와 위젯이 그 주위에 그리는 몇 개의 얇은 선으로 구성됩니다. 색상이나 모서리 둥글기를 변경하려면 `textarea`를 직접 스타일링하는 대신 이러한 CSS 변수를 설정하세요. 이 변수들은 테두리의 모든 부분을 한 번에 다시 스타일링하므로 모서리와 색상이 항상 일치합니다.

| Variable | What it changes | Default |
|---|---|---|
| `--fc-input-border-color` | 테두리 색상 | `#bfbfbf` |
| `--fc-input-border-color-focus` | 사용자가 입력 중일 때 테두리 색상 | `#555` |
| `--fc-input-border-radius` | 둥근 모서리의 반경 | `11px` |
| `--fc-input-border-start-start-radius` | 좌상단 사각형 모서리의 반경 (RTL 언어에서는 우상단) | `0` |

Add the CSS to the **Custom CSS** box on the [Widget Customization page](https://fastcomments.com/auth/my-account/customize-widget), or pass it with the `customCSS` option. You only need to set the variables you want to change.

CSS를 **Custom CSS** 상자에 추가하거나, `customCSS` 옵션으로 전달하세요. 변경하고 싶은 변수만 설정하면 됩니다.

You can also use the **Comment box border** helper right under the Custom CSS box, which writes this CSS for you.

또한 Custom CSS 상자 바로 아래에 있는 **Comment box border** 도우미를 사용하면 이 CSS를 자동으로 작성해 줍니다.

## Change the Border Color

[inline-code-attrs-start title = '테두리 색상'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## Change the Border Color While Typing

[inline-code-attrs-start title = '입력 중 테두리 색상'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## Round All Four Corners

By default the top-left corner is square. Set both radius variables to round all four corners the same:

기본적으로 좌상단 모서리는 사각형입니다. 두 반경 변수를 모두 설정하여 네 모서리를 동일하게 둥글게 만들 수 있습니다.

[inline-code-attrs-start title = '네 모서리 모두 둥글게'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## Square Corners

[inline-code-attrs-start title = '사각형 모서리'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## Match Your Brand

[inline-code-attrs-start title = '브랜드 색상 및 모서리'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## Different Colors in Dark Mode

When the widget is in dark mode it has the `dark` class, so you can set different values for dark mode:

다크 모드에서는 위젯에 `dark` 클래스가 적용되므로 다크 모드용으로 다른 값을 설정할 수 있습니다.

[inline-code-attrs-start title = '다크 모드 테두리 색상'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## Why Not Style the Text Area Directly?

The widget draws part of the comment box border itself, around the text area. If you set `border-color` or `border-radius` only on the `textarea`, those lines keep the default style and the border looks mismatched, for example a square line running through a rounded corner. The variables above change both at once.

위젯은 텍스트 영역 주위에 댓글 상자 테두리의 일부를 직접 그립니다. `textarea`에만 `border-color`나 `border-radius`를 설정하면 해당 선들은 기본 스타일을 유지해 테두리가 맞지 않게 보입니다. 예를 들어 둥근 모서리를 가로지르는 사각형 선이 나타날 수 있습니다. 위의 변수들은 두 가지를 한 번에 변경합니다.

---