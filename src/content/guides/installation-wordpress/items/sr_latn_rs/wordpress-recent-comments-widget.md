Widget Recent Comments prikazuje najnovije komentare objavljene na celom vašem sajtu. Koristan je u bočnim trakama, podnožjima ili bilo gde gde želite da istaknete svežu aktivnost kako biste podstakli dalje čitanje.

## Opcije

- **Title** (opciono): Naslov koji se prikazuje iznad liste. Podrazumevano je "Recent Comments".
- **Count** (opciono): Koliko komentara prikazati. Opseg od 1 do 50. Podrazumevano je 5.

## Kako da ga dodate

### Unutar posta ili stranice

U blok editoru dodajte blok **Shortcode** i nalepite:

[inline-code-attrs-start title = 'Shortcode za Recent Comments'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

Atribut `count` prihvata bilo koju vrednost između 1 i 50.

### U bočnoj traci ili podnožju (klasične teme)

Idite na **Appearance > Widgets** u WordPress administraciji. Iz blok ubacivača pretražite "FastComments" i izaberite **FastComments: Recent Comments**. Prevucite ga u bočnu traku, zaglavlje ili oblast podnožja, a zatim podesite naslov i broj iz panela widgeta.

### U blok temi (Full Site Editing)

Otvorite **Site Editor** pod **Appearance > Editor**. Idite na deo šablona gde widget treba da se pojavi, ubacite blok **Legacy Widget** i izaberite **FastComments: Recent Comments** iz padajuće liste.

## Rešavanje problema

Widget se renderuje tek nakon što je podešavanje FastComments završeno i kada je sačuvan tenant ID. Ako je oblast widgeta prazna, završite podešavanje pod **FastComments** u WordPress administraciji i ponovo učitajte stranicu.