Vidžet Najnovije diskusije prikazuje stranice na vašem sajtu sa najnovijom aktivnošću komentara. Koristan je za isticanje tema u kojima se i dalje dodaju komentari, tako da posjetioci mogu da se vrate u aktivne razgovore umjesto da dospiju na mirne stranice.

## Opcije

- **Naslov** (opcionalno): Naslov koji se prikazuje iznad liste. Podrazumijevano: "Najnovije diskusije".
- **Broj** (opcionalno): Koliko diskusija prikazati. Opseg 1 do 50. Podrazumijevano: 20.

## Kako ga dodati

### Unutar objave ili stranice

U editoru blokova dodajte blok **Shortcode** i zalijepite:

[inline-code-attrs-start title = 'Shortcode za Najnovije diskusije'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

Atribut `count` prihvata bilo koju vrijednost između 1 i 50.

### U bočnoj traci ili podnožju (klasične teme)

Idite u **Izgled > Widgeti** u WordPress administraciji. Iz ubacivača blokova potražite "FastComments" i odaberite **FastComments: Najnovije diskusije**. Prevucite ga u bočnu traku, zaglavlje ili podnožje, zatim konfigurirajte naslov i broj iz panela widgeta.

### U temi sa blokovima (uređivanje cijelog sajta)

Otvorite **Editor sajta** pod **Izgled > Editor**. Idite do dijela predloška gdje bi vidžet trebao da se pojavi, ubacite blok **Legacy Widget**, i iz padajućeg menija odaberite **FastComments: Najnovije diskusije**.

## Otklanjanje problema

Vidžet se prikazuje tek nakon što je podešavanje FastComments završeno i kada je tenant ID sačuvan. Ako je područje vidžeta prazno, dovršite podešavanje pod **FastComments** u WordPress administraciji i ponovo učitajte stranicu.

Ako redoslijed diskusija izgleda zastarjelo, provjerite da li su osnovne stranice završile sinhronizaciju u FastComments kontrolnoj tabli. Vidžet čita podatke uživo, pa svježe uvezeni komentari mogu potrajati da se pojave.