The Recent Comments widget displays the most recent comments posted across your entire site. It's useful in sidebars, footers, or anywhere you want to surface fresh activity to encourage further reading.

## Opcije

- **Title** (optional): The heading shown above the list. Defaults to "Recent Comments".
- **Count** (optional): How many comments to show. Range 1 to 50. Defaults to 5.

## Kako ga dodati

### Unutar objave ili stranice

U uređivaču blokova dodajte blok **Shortcode** i zalijepite:

[inline-code-attrs-start title = 'Kratka oznaka Nedavni komentari'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

The `count` attribute accepts any value between 1 and 50.

### U bočnoj traci ili podnožju (Classic Themes)

Idite na **Appearance > Widgets** u WordPress administraciji. Iz umetnika blokova potražite "FastComments" i odaberite **FastComments: Recent Comments**. Povucite ga u bočnu traku, header, ili područje podnožja, zatim konfigurirajte naslov i broj iz panela widgeta.

### U blok-temi (Full Site Editing)

Otvorite **Site Editor** pod **Appearance > Editor**. Navigirajte do dijela predloška gdje bi widget trebao biti prikazan, umetnite blok **Legacy Widget**, i iz padajućeg izbornika odaberite **FastComments: Recent Comments**.

## Rješavanje problema

Widget se prikazuje tek nakon što je FastComments postavljanje dovršeno i pohranjen je tenant ID. Ako je područje widgeta prazno, dovršite postavljanje pod **FastComments** u WordPress administraciji i ponovno učitajte stranicu.