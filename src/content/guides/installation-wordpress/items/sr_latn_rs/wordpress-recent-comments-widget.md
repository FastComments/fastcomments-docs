Vidžet Recent Comments prikazuje najnovije komentare postavljene na celom vašem sajtu. Koristan je u bočnim trakama, podnožjima ili bilo gde želite da istaknete svežu aktivnost kako biste podstakli dalje čitanje.

## Options

- **Title** (optional): Naslov koji se prikazuje iznad liste. Podrazumevano je "Recent Comments".
- **Count** (optional): Koliko komentara će biti prikazano. Opseg 1 do 50. Podrazumevano 5.

## How to Add It

### Inside a Post or Page

U uređivaču blokova dodajte blok **Shortcode** i nalepite:

[inline-code-attrs-start title = 'Shortcode za Recent Comments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

The `count` attribute accepts any value between 1 and 50.

### In a Sidebar or Footer (Classic Themes)

Idite na **Appearance > Widgets** u WordPress administraciji. Iz insertera blokova potražite "FastComments" i izaberite **FastComments: Recent Comments**. Prevucite ga u bočnu traku, zaglavlje ili oblast podnožja, zatim konfigurišite **Title** i **Count** iz panela vidžeta.

### In a Block Theme (Full Site Editing)

Otvorite **Site Editor** pod **Appearance > Editor**. Idite do dela šablona gde vidžet treba da se pojavi, umetnite blok **Legacy Widget** i iz padajućeg menija izaberite **FastComments: Recent Comments**.

## Troubleshooting

Vidžet se prikazuje tek nakon što je podešavanje FastComments kompletirano i tenant ID je sačuvan. Ako je područje za vidžet prazno, završite podešavanje pod **FastComments** u WordPress administraciji i ponovo učitajte stranicu.