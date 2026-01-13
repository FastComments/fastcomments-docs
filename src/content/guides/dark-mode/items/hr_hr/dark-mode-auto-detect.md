Prema zadanim postavkama, FastComments će automatski otkriti ima li vaša stranica tamnu pozadinu na temelju "udaljenosti od crne" u krugu boja.

Naši proizvodi daju sve od sebe u tome, međutim postoji gotovo beskonačno boja u krugu boja i mogu postojati scenariji u kojima aplikacija odabire korištenje tamnog načina rada kada to nije prikladno, i obrnuto. Ova dokumentacija pokriva kako imati precizniju kontrolu nad tim.

#### Tehnički detalji

Otkrivamo tamni način rada prolazeći kroz elemente na stranici prema gore od widgeta za komentare, tražeći tamnu pozadinu kada se widget inicijalno učita.

Za prebacivanje tamnog načina rada nakon ovog koraka, morate pozvati widget da ažurira svoju konfiguraciju. To je pokriveno u odjeljku `Ručna konfiguracija`.
