댓글 상자 주변의 테두리는 텍스트 영역 자체의 테두리와 위젯이 그 주위에 그리는 몇 개의 얇은 선으로 구성됩니다. 색상이나 모서리 둥글기를 변경하려면 `textarea`를 직접 스타일링하는 대신 다음 CSS 변수를 설정하세요. 이 변수들은 테두리의 모든 부분을 한 번에 재스타일링하므로 모서리와 색상이 항상 일치합니다.

| 변수 | 변경 내용 | 기본값 |
|---|---|---|
| `--fc-input-border-color` | 테두리 색상 | `#bfbfbf` |
| `--fc-input-border-color-focus` | 사용자가 입력하는 동안의 테두리 색상 | `#555` |
| `--fc-input-border-radius` | 둥근 모서리의 둥글기 | `11px` |
| `--fc-input-border-start-start-radius` | 왼쪽 위 사각형 모서리의 둥글기 (오른쪽에서 왼쪽 언어에서는 오른쪽 위) | `0` |

[Widget Customization page](https://fastcomments.com/auth/my-account/customize-widget)의 **Custom CSS** 상자에 CSS를 추가하거나, `customCSS` 옵션으로 전달하세요. 변경하려는 변수만 설정하면 됩니다.

Custom CSS 상자 바로 아래의 **Comment box border** 헬퍼를 사용할 수도 있는데, 이 헬퍼가 이 CSS를 대신 작성해 줍니다.

## 테두리 색상 변경

[inline-code-attrs-start title = '테두리 색상'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## 입력 중 테두리 색상 변경

[inline-code-attrs-start title = '입력 중 테두리 색상'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## 네 모서리 모두 둥글게 만들기

기본적으로 왼쪽 위 모서리는 사각형입니다. 네 모서리를 모두 동일하게 둥글게 만들려면 두 radius 변수를 모두 설정하세요:

[inline-code-attrs-start title = '네 모서리 모두 둥글게'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## 사각형 모서리

[inline-code-attrs-start title = '사각형 모서리'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## 브랜드에 맞추기

[inline-code-attrs-start title = '브랜드 색상 및 모서리'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## 다크 모드에서 다른 색상 사용

위젯이 다크 모드일 때는 `dark` 클래스가 적용되므로, 다크 모드에 대해 다른 값을 설정할 수 있습니다:

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

## 텍스트 영역을 직접 스타일링하면 안 되나요?

위젯은 텍스트 영역 주변의 댓글 상자 테두리 일부를 직접 그립니다. `textarea`에만 `border-color` 또는 `border-radius`를 설정하면, 그 선들은 기본 스타일을 유지하여 테두리가 어울리지 않게 보입니다. 예를 들어 둥근 모서리를 가로지르는 사각형 선이 나타날 수 있습니다. 위의 변수들은 두 부분을 한 번에 변경합니다.