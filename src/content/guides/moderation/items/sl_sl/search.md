Komentarje je mogoče iskati z naslednjo sintakso:

- Približno iskanje besed: `cats love`
- Natančno ujemanje fraze: `I love cats.`
- Natančno ujemanje celotnega komentarja: `exact="I love cats."`
  - Ujema le komentarje, katerih celotno besedilo je natanko ta vrednost (občutljivo na velike/male črke), ne pa komentarjev, ki to vrednost le vsebujejo.
- Po naslovu strani: `page:"Page Title"`
  - Podpira samodokončanje.
- Po URL-ju strani: `page:"https://example.com/some-page"`
  - Podpira samodokončanje.
- Po spletnem mestu/domeni: `site:mysite.com` ali `domain:othersite.com`
- Po uporabniku: `user:"Bob"`
  - Podpira samodokončanje.

---

Rezultate iskanja lahko delite z drugimi moderatorji ali skrbniki tako, da delite URL iz strani za moderacijo. Vrednost polja za iskanje
bo vključena v URL v vašem brskalniku, potem ko kliknete "Pojdi".