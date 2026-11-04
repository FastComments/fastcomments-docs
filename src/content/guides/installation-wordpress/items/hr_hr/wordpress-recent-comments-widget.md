Widget Nedavni komentari prikazuje najnovije komentare objavljene na cijeloj vašoj web-lokaciji. Koristan je u bočnim trakama, podnožjima ili bilo gdje gdje želite prikazati svježe aktivnosti kako biste potaknuli daljnje čitanje.

## Opcije

- **Naslov** (neobavezno): Naslov prikazan iznad popisa. Zadana vrijednost je "Nedavni komentari".
- **Broj** (neobavezno): Koliko komentara prikazati. Raspon od 1 do 50. Zadana vrijednost je 5.

## Kako ga dodati

### Unutar objave ili stranice

U uređivaču blokova dodajte blok **Shortcode** i zalijepite:

[inline-code-attrs-start title = 'Shortcode za Nedavne komentare'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

Atribut `count` prihvaća bilo koju vrijednost između 1 i 50.

### U bočnoj traci ili podnožju (klasične teme)

Idite na **Izgled > Widgeti** u svom WordPress administracijskom sučelju. Iz ubacivača blokova pretražite "FastComments" i odaberite **FastComments: Recent Comments**. Povucite ga u bočnu traku, zaglavlje ili podnožje, a zatim konfigurirajte naslov i broj iz panela widgeta.

### U blok temi (Full Site Editing)

Otvorite **Uređivač web-lokacije** pod **Izgled > Uređivač**. Dođite do dijela predloška gdje bi se widget trebao pojaviti, umetnite blok **Legacy Widget** i odaberite **FastComments: Recent Comments** iz padajućeg izbornika.

## Rješavanje problema

Widget se prikazuje tek nakon što je FastComments postavljanje dovršeno i ID stanara je pohranjen. Ako je područje widgeta prazno, dovršite postavljanje pod **FastComments** u WordPress administraciji i ponovno učitajte stranicu.