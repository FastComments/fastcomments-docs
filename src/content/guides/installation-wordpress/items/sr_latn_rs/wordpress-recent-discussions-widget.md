Widget Nedavne diskusije prikazuje stranice na vašem sajtu sa najnovijom aktivnošću komentara. Koristan je za isticanje niti kojima se i dalje dodaju komentari, tako da posetioci mogu da se vrate u aktivne razgovore umesto da dospeju na tihe stranice.

## Opcije

- **Naslov** (opciono): Naslov koji se prikazuje iznad liste. Podrazumevano: "Nedavne diskusije".
- **Broj** (opciono): Koliko diskusija da se prikaže. Opseg 1 do 50. Podrazumevano: 20.

## Kako ga dodati

### U objavi ili stranici

U blok editoru dodajte **Shortcode** blok i nalepite:

[inline-code-attrs-start title = 'Shortcode za Nedavne Diskusije'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

Atribut `count` prihvata bilo koju vrednost između 1 i 50.

### U bočnoj traci ili podnožju (klasične teme)

Idite na **Appearance > Widgets** u administratorskom delu WordPress-a. Iz ubacivača blokova potražite "FastComments" i izaberite **FastComments: Recent Discussions**. Prevucite ga u bočnu traku, header ili podnožje, zatim konfigurišite naslov i broj iz podešavanja widgeta.

### U blok temi (Full Site Editing)

Otvorte **Site Editor** pod **Appearance > Editor**. Idite do dela šablona gde widget treba da se pojavi, ubacite blok **Legacy Widget**, i iz padajućeg menija odaberite **FastComments: Recent Discussions**.

## Otklanjanje problema

Widget se prikazuje tek nakon što je FastComments podešavanje završeno i kada je sačuvan tenant ID. Ako je oblast widgeta prazna, završite podešavanje u sekciji **FastComments** u administratorskom delu WordPress-a i osvežite stranicu.

Ako izgled poređanja diskusija deluje zastarelo, proverite da li su osnovne stranice završile sinhronizaciju u FastComments dashboard-u. Widget čita žive podatke, pa se nedavno uvezeni komentari mogu pojaviti nakon kratkog vremena.