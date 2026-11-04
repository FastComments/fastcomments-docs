Widget Top Pages prikazuje najkomentarisane stranice na vašem sajtu. Koristan je za isticanje vašeg najprivlačnijeg sadržaja novim posetiocima i povećanje vremena provedenog na sajtu.

## Opcije

- **Title** (opciono): Naslov koji se prikazuje iznad liste. Podrazumevano je "Top Pages".

Widget Top Pages sam bira svoj raspored na osnovu dostupnih podataka i ne prihvata count atribut.

## Kako da ga dodate

### Unutar posta ili stranice

U blok editoru dodajte blok **Shortcode** i nalepite:

[inline-code-attrs-start title = 'Top Pages shortcode'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_top_pages]
[inline-code-end]

### U bočnoj traci ili podnožju (klasične teme)

Idite na **Appearance > Widgets** u vašem WordPress admin panelu. Iz blok ubacivača pretražite "FastComments" i izaberite **FastComments: Top Pages**. Prevucite ga u bočnu traku, zaglavlje ili oblast podnožja, a zatim podesite naslov iz panela widgeta.

### U blok temi (Full Site Editing)

Otvorite **Site Editor** pod **Appearance > Editor**. Idite do dela šablona gde widget treba da se pojavi, ubacite blok **Legacy Widget** i izaberite **FastComments: Top Pages** iz padajuće liste.

## Rešavanje problema

Widget se prikazuje tek nakon što je FastComments podešavanje završeno i tenant ID je sačuvan. Ako je oblast widgeta prazna, završite podešavanje pod **FastComments** u WordPress admin panelu i ponovo učitajte stranicu.