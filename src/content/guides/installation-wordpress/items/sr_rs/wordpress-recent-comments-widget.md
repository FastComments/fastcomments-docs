Vidžet Nedavni komentari prikazuje najnovije komentare postavljene na celom vašem sajtu. Koristan je u bočnim trakama, podnožjima ili bilo gde želite da istaknete svežu aktivnost kako biste podstakli dalje čitanje.

## Opcije

- **Naslov** (opciono): Naslov prikazan iznad liste. Podrazumevano: "Nedavni komentari".
- **Broj** (opciono): Koliko komentara prikazati. Opseg 1 do 50. Podrazumevano: 5.

## Kako ga dodati

### Unutar objave ili stranice

U bloku uređivača, dodajte blok **Kratki kod** i nalepite:

[inline-code-attrs-start title = 'Kratki kod za Nedavne komentare'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

Atribut `count` prihvata bilo koju vrednost između 1 i 50.

### U bočnoj traci ili podnožju (klasične teme)

Idite na **Izgled > Vidžeti** u vašem WordPress admin delu. Iz ubacivača blokova, potražite "FastComments" i izaberite **FastComments: Nedavni komentari**. Prevucite ga u bočnu traku, zaglavlje ili područje podnožja, zatim konfigurišite naslov i broj iz panela vidžeta.

### U temi zasnovanoj na blokovima (Uređivanje celog sajta)

Otvorite **Uređivač sajta** pod **Izgled > Uređivač**. Idite do dela predloška gde bi vidžet trebalo da se pojavi, ubacite blok **Nasleđeni vidžet**, i iz padajućeg menija izaberite **FastComments: Nedavni komentari**.

## Otklanjanje problema

Vidžet se prikazuje tek nakon što je podešavanje FastComments završeno i kada je tenant ID sačuvan. Ako je oblast vidžeta prazna, završite podešavanje u delu **FastComments** u WordPress administraciji i osvežite stranicu.