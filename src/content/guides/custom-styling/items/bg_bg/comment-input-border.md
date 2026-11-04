Рамката около полето за коментар се състои от собствения border на textarea плюс няколко тънки линии, които уиджетът изчертава около него. За да промените цвета ѝ или заоблеността на ъглите, задайте тези CSS променливи вместо да стилизирате `textarea` директно. Променливите преоформят всички части на рамката наведнъж, така че ъглите и цветовете винаги съвпадат.

| Променлива | Какво променя | Стойност по подразбиране |
|---|---|---|
| `--fc-input-border-color` | Цвят на рамката | `#bfbfbf` |
| `--fc-input-border-color-focus` | Цвят на рамката, докато потребителят пише | `#555` |
| `--fc-input-border-radius` | Заобленост на закръглените ъгли | `11px` |
| `--fc-input-border-start-start-radius` | Заобленост на квадратния горен ляв ъгъл (горен десен при езици с писане отдясно наляво) | `0` |

Добавете CSS в полето **Custom CSS** на [страницата за персонализиране на уиджета](https://fastcomments.com/auth/my-account/customize-widget), или го подайте чрез опцията `customCSS`. Трябва да зададете само променливите, които искате да промените.

Можете също да използвате помощника **Comment box border** точно под полето Custom CSS, който написва този CSS вместо вас.

## Промяна на цвета на рамката

[inline-code-attrs-start title = 'Цвят на рамката'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## Промяна на цвета на рамката при писане

[inline-code-attrs-start title = 'Цвят на рамката при писане'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## Заобляне на всичките четири ъгъла

По подразбиране горният ляв ъгъл е квадратен. Задайте двете променливи за радиус, за да заоблите всичките четири ъгъла еднакво:

[inline-code-attrs-start title = 'Заобляне на всичките четири ъгъла'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## Квадратни ъгли

[inline-code-attrs-start title = 'Квадратни ъгли'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## Съгласуване с вашата марка

[inline-code-attrs-start title = 'Цветове и ъгли на марката'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## Различни цветове в тъмен режим

Когато уиджетът е в тъмен режим, той има класа `dark`, така че можете да зададете различни стойности за тъмен режим:

[inline-code-attrs-start title = 'Цветове на рамката в тъмен режим'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## Защо не да стилизирате textarea директно?

Уиджетът изчертава част от рамката на полето за коментар сам, около текстовата област. Ако зададете `border-color` или `border-radius` само на `textarea`, тези линии запазват стила по подразбиране и рамката изглежда несъгласувана — например квадратна линия, минаваща през закръглен ъгъл. Горните променливи променят двете наведнъж.