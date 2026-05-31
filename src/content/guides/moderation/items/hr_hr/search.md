---
Komentare možete pretraživati pomoću sljedeće primjerne sintakse:

- Približna pretraga riječi: `cats love`
- Točno podudaranje fraze: `I love cats.`
- Točno podudaranje cijelog komentara: `exact="I love cats."`
  - Podudara se samo s komentarima čiji je cijeli tekst točno ova vrijednost (osjetljivo na velika i mala slova), umjesto komentara koji ju samo sadrže.
- Prema naslovu stranice: `page:"Page Title"`
  - Podržava automatsko dovršavanje.
- Prema URL-u stranice: `page:"https://example.com/some-page"`
  - Podržava automatsko dovršavanje.
- Po web-mjestu/domeni: `site:mysite.com` ili `domain:othersite.com`
- Po korisniku: `user:"Bob"`
  - Podržava automatsko dovršavanje.

Rezultate pretraživanja možete podijeliti s drugim moderatorima ili administratorima dijeljenjem URL-a stranice s stranice za moderaciju. Vrijednost polja za pretraživanje
bit će uključena u URL u vašem pregledniku nakon što pritisnete "Go".
---