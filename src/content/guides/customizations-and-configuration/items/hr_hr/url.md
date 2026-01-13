[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

Prilikom slanja obavijesnih e‑poruka ili prikazivanja komentara u korisničkim sučeljima kao što je stranica za moderaciju, korisno je moći povezati
komentar sa stranicom na kojoj se nalazi.

Ako URL ID nije uvijek stvarni ID, moramo URL pohraniti negdje drugdje. Za to služi svojstvo "url", definirano na sljedeći način.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

Uobičajen slučaj upotrebe je povezivanje niza komentara s identifikatorom, npr. člankom, a zatim povezivanje natrag na određenu stranicu, na primjer:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

URL se ne čisti od uobičajenih marketinških parametara. Prema zadanim postavkama, koji god je trenutni URL stranice, taj se URL pohranjuje uz komentar.