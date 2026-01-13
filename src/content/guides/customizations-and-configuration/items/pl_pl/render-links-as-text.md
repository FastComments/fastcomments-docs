---
Domyślnie FastComments będzie renderować linki w ten sposób: [https://exmaple.com](https://exmaple.com) - gdzie adres URL linku staje się klikalnym
elementem kotwicy HTML.

Niektóre serwisy mogą chcieć to wyłączyć, na przykład aby zniechęcić oszustów. Udostępniamy to poprzez ustawienie `Comment HTML Rendering Option` na `Links as Text`.

Można to dostosować bez kodu, na stronie dostosowywania widgetu, dla całej domeny, lub strony:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option']; selector = '.comment-html-rendering-mode'; title='Render Links as Text' app-screenshot-end]

---