Рамка вокруг поля комментария состоит из собственной границы текстового поля и нескольких тонких линий, которые виджет рисует вокруг него. Чтобы изменить её цвет или скругление углов, задайте эти CSS-переменные вместо стилизации `textarea` напрямую. Переменные изменяют стиль всей рамки сразу, поэтому углы и цвета всегда совпадают.

| Переменная | Что изменяет | Значение по умолчанию |
|---|---|---|
| `--fc-input-border-color` | Цвет рамки | `#bfbfbf` |
| `--fc-input-border-color-focus` | Цвет рамки во время ввода текста | `#555` |
| `--fc-input-border-radius` | Скругление скруглённых углов | `11px` |
| `--fc-input-border-start-start-radius` | Скругление квадратного левого верхнего угла (правого верхнего в языках с письмом справа налево) | `0` |

Добавьте CSS в поле **Custom CSS** на [странице настройки виджета](https://fastcomments.com/auth/my-account/customize-widget) или передайте его с помощью опции `customCSS`. Достаточно задать только те переменные, которые вы хотите изменить.

Вы также можете использовать помощник **Comment box border** прямо под полем Custom CSS, который напишет этот CSS за вас.

## Изменение цвета рамки

[inline-code-attrs-start title = 'Цвет рамки'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## Изменение цвета рамки во время ввода

[inline-code-attrs-start title = 'Цвет рамки во время ввода'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## Скругление всех четырёх углов

По умолчанию левый верхний угол квадратный. Задайте обе переменные радиуса, чтобы скруглить все четыре угла одинаково:

[inline-code-attrs-start title = 'Скругление всех четырёх углов'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## Квадратные углы

[inline-code-attrs-start title = 'Квадратные углы'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## Соответствие вашему бренду

[inline-code-attrs-start title = 'Цвета и углы бренда'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## Разные цвета в тёмном режиме

Когда виджет находится в тёмном режиме, он имеет класс `dark`, поэтому вы можете задать другие значения для тёмного режима:

[inline-code-attrs-start title = 'Цвета рамки в тёмном режиме'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## Почему не стилизовать текстовое поле напрямую?

Виджет рисует часть рамки поля комментария сам, вокруг текстового поля. Если вы зададите `border-color` или `border-radius` только для `textarea`, эти линии сохранят стиль по умолчанию, и рамка будет выглядеть несогласованно — например, квадратная линия, проходящая через скруглённый угол. Переменные выше изменяют и то, и другое одновременно.