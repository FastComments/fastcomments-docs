Widget "Nedavne rasprave" prikazuje stranice na vašem web-mjestu s najnovijom aktivnošću komentara. Koristan je za isticanje tema kojima se još uvijek dodaju komentari, kako bi posjetitelji mogli ponovno ući u aktivne razgovore umjesto da dospiju na tihe stranice.

## Opcije

- **Naslov** (neobavezno): Zaglavlje prikazano iznad popisa. Zadano je "Nedavne rasprave".
- **Broj** (neobavezno): Koliko rasprava prikazati. Raspon 1 do 50. Zadano je 20.

## Kako ga dodati

### Unutar objave ili stranice

U uređivaču blokova dodajte blok **Kratki kod** i zalijepite:

[inline-code-attrs-start title = 'Kratki kod za Nedavne rasprave'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

Atribut `count` prihvaća bilo koju vrijednost između 1 i 50.

### U bočnoj traci ili podnožju (klasične teme)

Idite na **Izgled > Widgeti** u WordPress administratorskom sučelju. U izborniku za umetanje blokova potražite "FastComments" i odaberite **FastComments: Nedavne rasprave**. Povucite ga u bočnu traku, zaglavlje ili podnožje, a zatim konfigurirajte naslov i broj iz panela widgeta.

### U blok-temi (uređivanje cijelog web-mjesta)

Otvorite **Uređivač web-mjesta** pod **Izgled > Uređivač**. Idite do dijela predloška gdje bi widget trebao biti prikazan, umetnite blok **Legacy Widget**, i odaberite **FastComments: Nedavne rasprave** iz padajućeg izbornika.

## Rješavanje problema

Widget se prikazuje tek nakon što je postavljanje FastComments završeno i kada je spremljen tenant ID. Ako je područje widgeta prazno, dovršite postavljanje pod **FastComments** u WordPress administratorskom sučelju i osvježite stranicu.

Ako redoslijed rasprava izgleda zastarjelo, provjerite jesu li temeljne stranice završile sinkronizaciju na nadzornoj ploči FastComments. Widget očitava žive podatke, pa nedavno uvezeni komentari mogu potrajati da se pojave.