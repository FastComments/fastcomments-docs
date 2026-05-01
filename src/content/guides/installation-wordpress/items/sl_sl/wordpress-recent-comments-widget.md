The Recent Comments widget displays the most recent comments posted across your entire site. It's useful in sidebars, footers, or anywhere you want to surface fresh activity to encourage further reading.

## Možnosti

- **Naslov** (neobvezno): Naslov, prikazan nad seznamom. Privzeto: "Nedavni komentarji".
- **Število** (neobvezno): Koliko komentarjev prikazati. Obseg 1 do 50. Privzeto: 5.

## Kako ga dodati

### V objavi ali strani

V urejevalniku blokov dodajte blok **kratka koda** in prilepite:

[inline-code-attrs-start title = 'Kratka koda za Nedavne komentarje'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_comments count="5"]
[inline-code-end]

Atribut `count` sprejme katerokoli vrednost med 1 in 50.

### V stranski vrstici ali podnožju (klasične teme)

Pojdite v **Videz > Pripomočki** v WordPress skrbništvu. Iz vstavljalca blokov poiščite "FastComments" in izberite **FastComments: Nedavni komentarji**. Povlecite ga v stransko vrstico, glavo ali podnožje in nato konfigurirajte naslov in število v panelu pripomočka.

### V temi z bloki (urejanje celotnega mesta)

Odprite **Urejevalnik mesta** pod **Videz > Urejevalnik**. Pomaknite se do dela predloge, kjer naj se pripomoček prikaže, vstavite blok **Legacy Widget** in iz spustnega seznama izberite **FastComments: Nedavni komentarji**.

## Odpravljanje težav

Pripomoček se prikaže šele, ko je namestitev FastComments zaključena in je shranjen tenant ID. Če je območje pripomočka prazno, dokončajte nastavitev v razdelku **FastComments** v WordPress skrbništvu in osvežite stran.