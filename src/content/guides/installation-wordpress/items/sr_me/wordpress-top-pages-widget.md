Widget Najviše komentarisane stranice prikazuje stranice sa najviše komentara na vašem sajtu. Koristan je za isticanje sadržaja koji najviše angažuje nove posjetioce i povećanje vremena provedenog na sajtu.

## Opcije

- **Naslov** (opciono): Naslov prikazan iznad liste. Podrazumevano je "Najviše komentarisane stranice".

Widget Najviše komentarisane stranice bira sopstveni raspored na osnovu dostupnih podataka i ne prihvata atribut count.

## Kako ga dodati

### Unutar objave ili stranice

U editoru blokova, dodajte blok **Shortcode** i zalijepite:

[inline-code-attrs-start title = 'Shortcode Najviše komentarisane stranice'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_top_pages]
[inline-code-end]

### U bočnoj traci ili podnožju (klasične teme)

Idite na **Izgled > Widgeti** u vašoj WordPress administraciji. Iz insertera blokova potražite "FastComments" i izaberite **FastComments: Najviše komentarisane stranice**. Prevucite ga u bočnu traku, zaglavlje ili podnožje, zatim podesite naslov iz panela za widget.

### U blok-temi (potpuno uređivanje sajta)

Otvorite **Editor sajta** pod **Izgled > Editor**. Idite do dela šablona gde widget treba da se pojavi, umetnite blok **Legacy Widget**, i iz padajućeg menija izaberite **FastComments: Najviše komentarisane stranice**.

## Otklanjanje problema

Widget se prikazuje tek nakon što je podešavanje FastComments završeno i tenant ID sačuvan. Ako je oblast widgeta prazna, dovršite podešavanje pod **FastComments** u WordPress administraciji i ponovo učitajte stranicu.