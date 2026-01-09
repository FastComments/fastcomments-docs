[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

Kada se šalju obaveštenja putem e‑pošte, ili se komentari prikazuju u korisničkim interfejsima poput stranice za moderaciju, korisno je omogućiti povezivanje
iz komentara na stranicu na kojoj se nalazi.

Ako URL ID nije uvek pravi ID, onda moramo URL sačuvati negde drugo. Za to služi svojstvo "url", definisano na sledeći način.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

Uobičajen slučaj upotrebe je povezivanje niza komentara sa identifikatorom, na primer člankom, i zatim povezivanje nazad na konkretnu stranicu, na primer:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

URL se ne čisti od uobičajenih marketinških parametara. Podrazumevano, šta god da je trenutni URL stranice, taj URL se čuva uz komentar.