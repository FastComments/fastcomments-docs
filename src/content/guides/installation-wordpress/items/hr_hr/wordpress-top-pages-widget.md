Najpopularnije stranice widget prikazuje stranice s najviše komentara na vašoj web-lokaciji. Koristan je za isticanje sadržaja koji najviše angažira nove posjetitelje i za povećanje vremena provedenog na stranici.

## Opcije

- **Naslov** (neobavezno): Zaglavlje prikazano iznad popisa. Zadano je "Najpopularnije stranice".

Widget Najpopularnije stranice odabire vlastiti izgled na temelju dostupnih podataka i ne prihvaća atribut count.

## Kako ga dodati

### Unutar objave ili stranice

U uređivaču blokova dodajte blok **Shortcode** i zalijepite:

[inline-code-attrs-start title = 'Shortcode za Najpopularnije stranice'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_top_pages]
[inline-code-end]

### U bočnoj traci ili podnožju (klasične teme)

Idite na **Izgled > Widgeti** u WordPress administraciji. Iz umetnika blokova potražite "FastComments" i odaberite **FastComments: Najpopularnije stranice**. Povucite ga u bočnu traku, zaglavlje ili područje podnožja, zatim postavite naslov iz panela widgeta.

### U temi s blokovima (potpuno uređivanje web-mjesta)

Otvorite **Uređivač web-mjesta** pod **Izgled > Uređivač**. Idite do dijela predloška gdje bi widget trebao biti, umetnite blok **Legacy Widget**, i iz padajućeg izbornika odaberite **FastComments: Najpopularnije stranice**.

## Rješavanje problema

Widget se prikazuje tek nakon što je postavljanje FastComments dovršeno i pohranjen je tenant ID. Ako je područje za widget prazno, dovršite postavljanje pod **FastComments** u WordPress administraciji i osvježite stranicu.