Rub oko okvira za komentar sastoji se od vlastitog ruba tekstualnog područja plus nekoliko tankih linija koje widget crta oko njega. Za promjenu njegove boje ili zaobljenosti kutova, postavite ove CSS varijable umjesto da stilizirate `textarea` izravno. Varijable ponovno stiliziraju svaki dio ruba odjednom, tako da se kutovi i boje uvijek podudaraju.

| Varijabla | Što mijenja | Zadano |
|---|---|---|
| `--fc-input-border-color` | Boja ruba | `#bfbfbf` |
| `--fc-input-border-color-focus` | Boja ruba dok korisnik tipka | `#555` |
| `--fc-input-border-radius` | Zaobljenost zaobljenih kutova | `11px` |
| `--fc-input-border-start-start-radius` | Zaobljenost kvadratnog gornjeg lijevog kuta (gornjeg desnog u jezicima s desna na lijevo) | `0` |

Dodajte CSS u okvir **Custom CSS** na [Widget Customization page](https://fastcomments.com/auth/my-account/customize-widget), ili ga proslijedite putem opcije `customCSS`. Trebate postaviti samo varijable koje želite promijeniti.

Također možete koristiti pomoćnik **Comment box border** odmah ispod okvira Custom CSS, koji za vas generira ovaj CSS.

## Promijeni boju ruba

[inline-code-attrs-start title = 'Boja okvira'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## Promijeni boju ruba tijekom tipkanja

[inline-code-attrs-start title = 'Boja okvira tijekom tipkanja'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## Zaobli sva četiri kuta

Prema zadanim postavkama gornji lijevi kut je kvadrat. Postavite obje varijable radijusa da zaokružite sva četiri kuta jednako:

[inline-code-attrs-start title = 'Zaobli sva četiri kuta'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## Kvadratni kutovi

[inline-code-attrs-start title = 'Kvadratni kutovi'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## Uskladite s vašim brendom

[inline-code-attrs-start title = 'Boje brenda i kutovi'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## Različite boje u tamnom načinu

Kada je widget u tamnom načinu, ima klasu `dark`, pa možete postaviti različite vrijednosti za tamni način:

[inline-code-attrs-start title = 'Boje rubova tamnog načina'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## Zašto ne stilizirati tekstualno područje izravno?

Widget crta dio ruba okvira za komentar sam, oko tekstualnog područja. Ako postavite `border-color` ili `border-radius` samo na `textarea`, te linije zadržavaju zadani stil i rub izgleda neusklađeno, na primjer kvadratna linija koja prolazi kroz zaobljeni kut. Varijable iznad mijenjaju oba odjednom.