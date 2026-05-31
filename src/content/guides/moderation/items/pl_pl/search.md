Komentarze można przeszukiwać za pomocą następującej przykładowej składni:

- Wyszukiwanie przybliżone (fuzzy): `cats love`
- Dopasowanie dokładnej frazy: `I love cats.`
- Dokładne dopasowanie całego komentarza: `exact="I love cats."`
  - Pasuje tylko do komentarzy, których cały tekst jest dokładnie tą wartością (z uwzględnieniem wielkości liter), a nie do komentarzy, które jedynie ją zawierają.
- Po tytule strony: `page:"Page Title"`
  - Obsługuje autouzupełnianie.
- Po adresie URL strony: `page:"https://example.com/some-page"`
  - Obsługuje autouzupełnianie.
- Po witrynie/domenie: `site:mysite.com` lub `domain:othersite.com`
- Po użytkowniku: `user:"Bob"`
  - Obsługuje autouzupełnianie.

Wyniki wyszukiwania możesz udostępnić innym moderatorom lub administratorom, udostępniając adres URL strony z poziomu strony moderacji. Wartość pola
wyszukiwania zostanie dołączona do adresu URL w Twojej przeglądarce po naciśnięciu "Szukaj".