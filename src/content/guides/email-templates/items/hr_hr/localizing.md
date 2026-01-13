FastComments je lokalizirana platforma. Svi naši widgeti, e-mailovi i obavijesti su lokalizirani.

Lokalizirano znači da prikazujemo različiti jezik i formatiranje, ovisno o lokaciji korisnika i njegovom preferiranom jeziku. To određujemo na temelju informacija koje nam pruža preglednik korisnika.

Tekst u e-mailu možemo prilagoditi odlaskom na karticu `Translations`, odabirom `Locale` i uređivanjem teksta. Tekst koji je promijenjen u odnosu na zadani je istaknut u korisničkom sučelju. Možete se prebacivati između lokala i spremiti na kraju, bez gubitka promjena.

Lokalizirani tekst se pristupa putem objekta `TEXT`, na primjer: `<%= TEXT.INTRO %>`.

### Napomena o SSO

Za SSO integracije, ako `locale` nije naveden, on će se ažurirati svaki put kada korisnik pristupi widgetu za komentare s drugačijim lokalom. To znači da se njihova jezična preferencija automatski ažurira, i budući e-mailovi bit će poslani na taj lokal.

To se također može postaviti ručno pružajući `locale` u SSO payloadu.