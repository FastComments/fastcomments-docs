Widget Recent Discussions prikazuje stranice na vašem sajtu sa najnovijom aktivnošću komentara. Koristan je za isticanje niti u koje se i dalje dodaju komentari, tako da posjetioci mogu ponovo uskočiti u aktivne razgovore umjesto da završe na tihim stranicama.

## Options

- **Title** (optional): Naslov prikazan iznad liste. Podrazumijevano: "Nedavne diskusije".
- **Count** (optional): Koliko diskusija prikazati. Opseg 1 do 50. Podrazumijevano: 20.

## How to Add It

### Inside a Post or Page

U blok uređivaču, dodajte **Shortcode** blok i zalijepite:

[inline-code-attrs-start title = 'Shortcode za Nedavne diskusije'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
[fastcomments_recent_discussions count="20"]
[inline-code-end]

Atribut `count` prihvata bilo koju vrijednost između 1 i 50.

### In a Sidebar or Footer (Classic Themes)

Idite na **Izgled > Widgeti** u vašem WordPress adminu. Iz umetnika blokova potražite "FastComments" i izaberite **FastComments: Recent Discussions**. Prevucite ga u bočnu traku, zaglavlje ili podnožje, zatim podesite naslov i broj iz panela widgeta.

### In a Block Theme (Full Site Editing)

Otvorite **Uređivač sajta** pod **Izgled > Uređivač**. Navigirajte do dijela predloška gdje widget treba da se pojavi, umetnite **Legacy Widget** blok i iz padajućeg menija odaberite **FastComments: Recent Discussions**.

## Troubleshooting

Widget se prikazuje tek nakon što je podešavanje FastComments završeno i kada je sačuvan tenant ID. Ako je područje widgeta prazno, dovršite podešavanje pod **FastComments** u WordPress adminu i ponovo učitajte stranicu.

Ako redoslijed diskusija izgleda zastarjelo, provjerite jesu li odgovarajuće stranice završile sinhronizaciju na FastComments kontrolnoj tabli. Widget čita žive podatke, tako da svježe uvezeni komentari mogu trebati trenutak da postanu vidljivi.