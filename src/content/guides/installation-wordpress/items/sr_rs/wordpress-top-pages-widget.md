Widget Najpopularnije stranice prikazuje stranice sa najviše komentara na vašem sajtu. Koristan je za isticanje najinteresantnijeg sadržaja novim posetiocima i za povećanje vremena provedenog na sajtu.

## Opcije

- **Naslov** (opciono): Naslov koji se prikazuje iznad liste. Podrazumevano je "Najpopularnije stranice".

Widget Najpopularnije stranice bira svoj raspored na osnovu dostupnih podataka i ne prihvata atribut count.

## Kako ga dodati

### Unutar objave ili stranice

U editoru blokova dodajte blok **Kratki kod** i nalepite:

[inline-code-attrs-start title = 'Kratki kod za Najpopularnije stranice'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_top_pages]
[inline-code-end]

### U bočnoj traci ili podnožju (klasične teme)

Idite na **Izgled > Vidžeti** u vašoj WordPress administraciji. Iz blok insertera potražite "FastComments" i izaberite **FastComments: Najpopularnije stranice**. Prevucite ga u bočnu traku, zaglavlje ili podnožje, zatim podesite naslov iz panela vidžeta.

### U temi blokova (puno uređivanje sajta)

Otvorite **Uređivač sajta** pod **Izgled > Uređivač**. Idite do dela šablona gde bi vidžet trebao da se pojavi, umetnite blok **Legacy Widget**, i iz padajućeg menija izaberite **FastComments: Najpopularnije stranice**.

## Rešavanje problema

Vidžet se prikazuje tek nakon što je podešavanje FastComments završeno i tenant ID je sačuvan. Ako je oblast vidžeta prazna, završite podešavanje u **FastComments** u WordPress administraciji i osvežite stranicu.