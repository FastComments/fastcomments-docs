Рамка навколо поля коментаря складається з власної межі текстової області та кількох тонких ліній, які віджет малює навколо неї. Щоб змінити її колір або заокругленість кутів, задайте ці CSS-змінні замість стилізації `textarea` безпосередньо. Змінні одночасно змінюють стиль усіх частин рамки, тому кути та кольори завжди збігаються.

| Змінна | Що змінює | Значення за замовчуванням |
|---|---|---|
| `--fc-input-border-color` | Колір рамки | `#bfbfbf` |
| `--fc-input-border-color-focus` | Колір рамки під час введення тексту | `#555` |
| `--fc-input-border-radius` | Заокругленість заокруглених кутів | `11px` |
| `--fc-input-border-start-start-radius` | Заокругленість квадратного верхнього лівого кута (верхнього правого в мовах з написанням справа наліво) | `0` |

Додайте CSS у поле **Custom CSS** на [сторінці налаштування віджета](https://fastcomments.com/auth/my-account/customize-widget) або передайте його через опцію `customCSS`. Потрібно задати лише ті змінні, які ви хочете змінити.

Ви також можете скористатися помічником **Comment box border** одразу під полем Custom CSS, який напише цей CSS за вас.

## Зміна кольору рамки

[inline-code-attrs-start title = 'Колір рамки'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## Зміна кольору рамки під час введення тексту

[inline-code-attrs-start title = 'Колір рамки під час введення тексту'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## Заокруглення всіх чотирьох кутів

За замовчуванням верхній лівий кут є квадратним. Задайте обидві змінні радіуса, щоб заокруглити всі чотири кути однаково:

[inline-code-attrs-start title = 'Заокруглення всіх чотирьох кутів'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## Квадратні кути

[inline-code-attrs-start title = 'Квадратні кути'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## Відповідність вашому бренду

[inline-code-attrs-start title = 'Кольори та кути бренду'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## Різні кольори в темному режимі

Коли віджет перебуває в темному режимі, він має клас `dark`, тому ви можете задати інші значення для темного режиму:

[inline-code-attrs-start title = 'Кольори рамки в темному режимі'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## Чому не варто стилізувати текстову область безпосередньо?

Віджет сам малює частину рамки поля коментаря, навколо текстової області. Якщо ви задасте `border-color` або `border-radius` лише для `textarea`, ці лінії збережуть стандартний стиль, і рамка виглядатиме неузгоджено — наприклад, квадратна лінія проходитиме через заокруглений кут. Наведені вище змінні змінюють обидва аспекти одночасно.