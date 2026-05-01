Widget Najnoviji komentari prikazuje najnovije komentare objavljene na cijelom vašem sajtu. Koristan je u bočnim trakama, podnožjima ili bilo gdje želite istaknuti svježu aktivnost kako biste potaknuli daljnje čitanje.

## Opcije

- **Naslov** (opcionalno): Zaglavlje prikazano iznad liste. Zadano je "Najnoviji komentari".
- **Broj** (opcionalno): Koliko komentara prikazati. Raspon 1 do 50. Zadano je 5.

## Kako ga dodati

### Unutar objave ili stranice

U blok editoru dodajte blok **Shortcode** i zalijepite:

[inline-code-attrs-start title = 'Shortcode za Najnovije komentare'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

The `count` attribute accepts any value between 1 and 50.

### U bočnoj traci ili podnožju (klasične teme)

Idite na **Izgled > Widgeti** u WordPress administraciji. Iz ubacivača blokova potražite "FastComments" i odaberite **FastComments: Najnoviji komentari**. Prevucite ga u bočnu traku, zaglavlje ili područje podnožja, zatim podesite naslov i broj iz panela widgeta.

### U blok temi (uređivanje cijelog sajta)

Otvorite **Uređivač sajta** pod **Izgled > Uređivač**. Navigirajte do dijela šablona gdje widget treba da se pojavi, umetnite blok **Legacy Widget**, i iz padajućeg menija odaberite **FastComments: Najnoviji komentari**.

## Rješavanje problema

Widget se prikazuje tek nakon što je podešavanje FastComments dovršeno i tenant ID je pohranjen. Ako je područje widgeta prazno, završite podešavanje pod **FastComments** u WordPress administraciji i ponovo učitajte stranicu.