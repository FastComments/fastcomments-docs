Ивица око поља за коментар састоји се од сопствене ивице текстуалног поља и неколико танких линија које виџет црта око њега. Да бисте променили њену боју или заобљеност углова, поставите ове CSS променљиве уместо да директно стилизујете `textarea`. Променљиве истовремено рестилизују сваки део ивице, тако да се углови и боје увек поклапају.

| Променљива | Шта мења | Подразумевано |
|---|---|---|
| `--fc-input-border-color` | Боја ивице | `#bfbfbf` |
| `--fc-input-border-color-focus` | Боја ивице док корисник куца | `#555` |
| `--fc-input-border-radius` | Заобљеност заобљених углова | `11px` |
| `--fc-input-border-start-start-radius` | Заобљеност квадратног горњег левог угла (горњи десни у језицима који се пишу с десна на лево) | `0` |

Додајте CSS у поље **Прилагођени CSS** на [страници за прилагођавање виџета](https://fastcomments.com/auth/my-account/customize-widget), или га проследите путем опције `customCSS`. Потребно је само да поставите променљиве које желите да промените.

Такође можете користити помоћник **Ивица поља за коментар** одмах испод поља за прилагођени CSS, који овај CSS пише уместо вас.

## Промена боје ивице

[inline-code-attrs-start title = 'Боја ивице'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## Промена боје ивице током куцања

[inline-code-attrs-start title = 'Боја ивице током куцања'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## Заобљавање сва четири угла

Подразумевано је горњи леви угао квадратан. Поставите обе променљиве полупречника да бисте заобљили сва четири угла на исти начин:

[inline-code-attrs-start title = 'Заобљавање сва четири угла'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## Квадратни углови

[inline-code-attrs-start title = 'Квадратни углови'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## Усклађивање са вашим брендом

[inline-code-attrs-start title = 'Боје и углови бренда'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## Различите боје у тамном режиму

Када је виџет у тамном режиму, има класу `dark`, тако да можете поставити различите вредности за тамни режим:

[inline-code-attrs-start title = 'Боје ивице у тамном режиму'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## Зашто не стилизувати текстуално поље директно?

Виџет сам црта део ивице поља за коментар, око текстуалног поља. Ако поставите `border-color` или `border-radius` само на `textarea`, те линије задржавају подразумевани стил и ивица изгледа неусклађено, на пример квадратна линија која пролази кроз заобљени угао. Горње променљиве мењају обоје одједном.