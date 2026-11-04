Pripomoček Recent Discussions prikazuje strani na vašem spletnem mestu z najnovejšo aktivnostjo komentarjev. Uporaben je za izpostavljanje niti, v katere se še vedno dodaja vsebina, tako da lahko obiskovalci skočijo nazaj v aktivne pogovore namesto da pristanejo na mirujočih straneh.

## Možnosti

- **Naslov** (neobvezno): Naslov, prikazan nad seznamom. Privzeto "Recent Discussions".
- **Število** (neobvezno): Koliko razprav prikazati. Obseg od 1 do 50. Privzeto 20.

## Kako ga dodati

### Znotraj objave ali strani

V urejevalniku blokov dodajte blok **Shortcode** in prilepite:

[inline-code-attrs-start title = 'Shortcode za Recent Discussions'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

Atribut `count` sprejme katero koli vrednost med 1 in 50.

### V stranski vrstici ali nogi (klasične teme)

Pojdite na **Appearance > Widgets** v skrbniškem vmesniku WordPress. Iz vstavljalnika blokov poiščite "FastComments" in izberite **FastComments: Recent Discussions**. Povlecite ga v stransko vrstico, glavo ali nogo, nato pa nastavite naslov in število v plošči pripomočka.

### V blokovni temi (polno urejanje spletnega mesta)

Odprite **Site Editor** pod **Appearance > Editor**. Pomaknite se do dela predloge, kjer naj se pripomoček prikaže, vstavite blok **Legacy Widget** in iz spustnega seznama izberite **FastComments: Recent Discussions**.

## Odpravljanje težav

Pripomoček se prikaže šele, ko je nastavitev FastComments dokončana in je shranjen ID najemnika. Če je območje pripomočka prazno, dokončajte nastavitev pod **FastComments** v skrbniškem vmesniku WordPress in ponovno naložite stran.

Če je razvrščanje razprav videti zastarelo, preverite, ali so osnovne strani dokončale sinhronizacijo v nadzorni plošči FastComments. Pripomoček bere podatke v živo, zato lahko sveže uvoženi komentarji potrebujejo trenutek, da se pojavijo.