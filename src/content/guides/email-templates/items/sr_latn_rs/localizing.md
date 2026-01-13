FastComments je lokalizovana platforma. Svi naši widgeti, imejlovi i obaveštenja su lokalizovani.

Lokalizovano znači da prikazujemo drugačiji jezik i formatiranje, zasnovano na lokaciji korisnika i preferiranom jeziku. Ovo određujemo na osnovu informacija koje nam pruža korisnikov pregledač.

Možemo prilagoditi tekst u imejlu tako što ćemo otići na karticu `Translations`, odabrati `Locale` i izmeniti tekst. Tekst koji je promenjen u odnosu na podrazumevani je označen u UI. Možete se prebacivati između locale-a i sačuvati na kraju, bez gubitka izmena.

Lokalizovani tekst se pristupa preko `TEXT` objekta, na primer: `<%= TEXT.INTRO %>`.

### Napomena o SSO

Za SSO integracije, ako `locale` nije naveden, on će se ažurirati svaki put kada korisnik pristupi widgetu za komentare sa drugim locale-om. To znači da se njihovo jezičko podešavanje automatski ažurira i budući imejlovi će biti poslati na tom locale-u.

Ovo se takođe može ručno podesiti pružanjem `locale` u SSO payload-u.