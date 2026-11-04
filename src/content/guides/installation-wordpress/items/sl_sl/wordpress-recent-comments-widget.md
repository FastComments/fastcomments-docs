Widget Nedavni komentarji prikazuje najnovejše komentarje, objavljene na vašem celotnem spletnem mestu. Uporaben je v stranskih vrsticah, nogah ali kjer koli, kjer želite pokazati svežo dejavnost in spodbuditi nadaljnje branje.

## Možnosti

- **Naslov** (neobvezno): Naslov, prikazan nad seznamom. Privzeto "Nedavni komentarji".
- **Števec** (neobvezno): Koliko komentarjev prikazati. Območje od 1 do 50. Privzeto 5.

## Kako ga dodati

### Znotraj objave ali strani

V urejevalniku blokov dodajte blok **Shortcode** in prilepite:

[inline-code-attrs-start title = 'Kratka koda Nedavni komentarji'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

Atribut `count` sprejme katero koli vrednost med 1 in 50.

### V stranski vrstici ali nogi (klasične teme)

Pojdite na **Videz > Widgeti** v skrbniškem vmesniku WordPress. Iz vstavljalnika blokov poiščite "FastComments" in izberite **FastComments: Nedavni komentarji**. Povlecite ga v stransko vrstico, glavo ali nogo, nato pa nastavite naslov in števec v plošči widgeta.

### V blokovni temi (polno urejanje spletnega mesta)

Odprite **Urejevalnik spletnega mesta** pod **Videz > Urejevalnik**. Pomaknite se do dela predloge, kjer naj se widget prikaže, vstavite blok **Legacy Widget** in iz spustnega seznama izberite **FastComments: Nedavni komentarji**.

## Odpravljanje težav

Widget se prikaže šele, ko je nastavitev FastComments zaključena in je ID najemnika shranjen. Če je območje widgeta prazno, dokončajte nastavitev pod **FastComments** v skrbniškem vmesniku WordPress in ponovno naložite stran.