Vidžet Nedavne diskusije prikazuje stranice na vašem sajtu sa najnovijom aktivnošću komentara. Koristan je za isticanje tema u koje se i dalje dodaju komentari, tako da posetioci mogu da se vrate u aktivne razgovore umesto da dospeju na tihe stranice.

## Opcije

- **Title** (opciono): Naslov koji se prikazuje iznad liste. Podrazumevano je "Nedavne diskusije".
- **Count** (opciono): Koliko diskusija prikazati. Opseg 1 do 50. Podrazumevano je 20.

## Kako ga dodati

### Unutar objave ili stranice

U editoru blokova, dodajte **Shortcode** blok i nalepite:

[inline-code-attrs-start title = 'Shortcode za Nedavne diskusije'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

Atribut `count` prihvata bilo koju vrednost između 1 i 50.

### U bočnoj traci ili podnožju (klasične teme)

Idite na **Appearance > Widgets** u WordPress administraciji. Iz blok insertera potražite "FastComments" i izaberite **FastComments: Recent Discussions**. Prevucite ga u bočnu traku, zaglavlje ili podnožje, zatim podesite naslov i broj iz panela vidžeta.

### U blok-temi (Full Site Editing)

Otvorite **Site Editor** pod **Appearance > Editor**. Navigirajte do dela šablona gde vidžet treba da se pojavi, umetnite blok **Legacy Widget**, i iz padajućeg menija izaberite **FastComments: Recent Discussions**.

## Otklanjanje problema

Vidžet se prikazuje tek nakon što je FastComments podešavanje završeno i kada je tenant ID sačuvan. Ako je oblast vidžeta prazna, završite podešavanje pod **FastComments** u WordPress administraciji i osvežite stranicu.

Ako raspored diskusija deluje zastarelo, proverite da li su osnovne stranice završile sinhronizaciju u FastComments dashboard-u. Vidžet čita žive podatke, pa se sveže uvezani komentari mogu pojaviti sa malim zakašnjenjem.