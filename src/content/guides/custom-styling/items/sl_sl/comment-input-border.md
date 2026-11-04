Obroba okoli polja za komentar je sestavljena iz lastne obrobe besedilnega polja in nekaj tankih črt, ki jih vtičnik nariše okoli njega. Če želite spremeniti njeno barvo ali zaobljenost kotov, nastavite te CSS spremenljivke namesto da oblikujete `textarea` neposredno. Spremenljivke hkrati preoblikujejo celotno obrobo, zato se koti in barve vedno ujemajo.

| Spremenljivka | Kaj spreminja | Privzeto |
|---|---|---|
| `--fc-input-border-color` | Barva obrobe | `#bfbfbf` |
| `--fc-input-border-color-focus` | Barva obrobe med tipkanjem | `#555` |
| `--fc-input-border-radius` | Zaobljenost zaobljenih kotov | `11px` |
| `--fc-input-border-start-start-radius` | Zaobljenost kvadratnega zgornjega levega kota (zgornji desni v jezikih, ki pišejo od desne proti levi) | `0` |

CSS dodajte v polje **Custom CSS** na [strani za prilagoditev vtičnika](https://fastcomments.com/auth/my-account/customize-widget), ali ga posredujte z možnostjo `customCSS`. Nastaviti morate le spremenljivke, ki jih želite spremeniti.

Prav tako lahko uporabite pomočnik **Comment box border** tik pod poljem Custom CSS, ki ta CSS zapiše namesto vas.

## Sprememba barve obrobe

[inline-code-attrs-start title = 'Barva obrobe'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## Sprememba barve obrobe med tipkanjem

[inline-code-attrs-start title = 'Barva obrobe med tipkanjem'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## Zaobljenje vseh štirih kotov

Privzeto je zgornji levi kot kvadraten. Nastavite obe spremenljivki za polmer, da zaoblite vse štiri kote enako:

[inline-code-attrs-start title = 'Zaobljenje vseh štirih kotov'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## Kvadratni koti

[inline-code-attrs-start title = 'Kvadratni koti'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## Prilagoditev vaši blagovni znamki

[inline-code-attrs-start title = 'Barve in koti blagovne znamke'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## Različne barve v temnem načinu

Ko je vtičnik v temnem načinu, ima razred `dark`, zato lahko za temni način nastavite različne vrednosti:

[inline-code-attrs-start title = 'Barve obrobe v temnem načinu'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## Zakaj ne oblikovati besedilnega polja neposredno?

Vtičnik del obrobe polja za komentar nariše sam, okoli besedilnega polja. Če nastavite `border-color` ali `border-radius` le na `textarea`, te črte obdržijo privzeti slog in obroba izgleda neusklajeno, na primer kvadratna črta, ki teče skozi zaobljeni kot. Zgornje spremenljivke spremenijo oboje hkrati.