Komentari se mogu pretraživati pomoću sledeće sintakse:

- Pretraga po sličnosti (fuzzy): `cats love`
- Tačno podudaranje fraze: `I love cats.`
- Tačno podudaranje celog komentara: `exact="I love cats."`
  - Usklađuje samo komentare čiji je ceo tekst tačno ova vrednost (osetljivo na velika/mala slova), a ne komentare koji je samo sadrže.
- Po naslovu stranice: `page:"Page Title"`
  - Podržava automatsko dovršavanje.
- Po URL-u stranice: `page:"https://example.com/some-page"`
  - Podržava automatsko dovršavanje.
- Po sajtu/domeni: `site:mysite.com` or `domain:othersite.com`
- Po korisniku: `user:"Bob"`
  - Podržava automatsko dovršavanje.

Možete podeliti rezultate pretrage sa drugim moderatorima ili administratorima tako što ćete podeliti URL stranice sa stranice za moderaciju. Vrednost polja za pretragu biće uključena u URL u vašem pregledaču nakon što kliknete "Go".