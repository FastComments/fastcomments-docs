Okvir oko polja za komentar čine sami okvir tekstualnog područja plus nekoliko tankih linija koje vidžet iscrtava oko njega. Da biste promenili boju ili zaobljenost uglova, postavite ove CSS promenljive umesto da stilizujete `textarea` direktno. Promenljive restilizuju sve delove okvira odjednom, tako da se uglovi i boje uvek poklapaju.

| Promenljiva | Šta menja | Podrazumevano |
|---|---|---|
| `--fc-input-border-color` | Boja okvira | `#bfbfbf` |
| `--fc-input-border-color-focus` | Boja okvira dok korisnik kuca | `#555` |
| `--fc-input-border-radius` | Zaobljenost zaobljenih uglova | `11px` |
| `--fc-input-border-start-start-radius` | Zaobljenost kvadratnog gornjeg levog ugla (gornji desni u jezicima koji se pišu s desna na levo) | `0` |

Dodajte CSS u polje **Custom CSS** na [stranici za prilagođavanje vidžeta](https://fastcomments.com/auth/my-account/customize-widget), ili ga prosledite kroz opciju `customCSS`. Potrebno je da postavite samo promenljive koje želite da promenite.

Takođe možete koristiti pomoćnik **Comment box border** odmah ispod polja Custom CSS, koji ovaj CSS napiše umesto vas.

## Promena boje okvira

[inline-code-attrs-start title = 'Boja okvira'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## Promena boje okvira dok kucate

[inline-code-attrs-start title = 'Boja okvira dok kucate'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## Zaobljavanje sva četiri ugla

Podrazumevano je gornji levi ugao kvadratan. Postavite obe promenljive za poluprečnik da biste zaoblili sva četiri ugla isto:

[inline-code-attrs-start title = 'Zaobljavanje sva četiri ugla'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## Kvadratni uglovi

[inline-code-attrs-start title = 'Kvadratni uglovi'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## Usklađivanje sa brendom

[inline-code-attrs-start title = 'Boje i uglovi brenda'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## Različite boje u tamnom režimu

Kada je vidžet u tamnom režimu, ima `dark` klasu, tako da možete postaviti različite vrednosti za tamni režim:

[inline-code-attrs-start title = 'Boje okvira u tamnom režimu'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## Zašto ne stilizovati tekstualno područje direktno?

Vidžet sam iscrtava deo okvira polja za komentar, oko tekstualnog područja. Ako postavite `border-color` ili `border-radius` samo na `textarea`, te linije zadržavaju podrazumevani stil i okvir deluje neusklađeno, na primer kvadratna linija koja prolazi kroz zaobljeni ugao. Gore navedene promenljive menjaju oba odjednom.