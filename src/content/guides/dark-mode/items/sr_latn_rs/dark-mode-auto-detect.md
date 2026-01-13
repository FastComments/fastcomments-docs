Podrazumevano, FastComments će automatski otkriti da li vaš sajt ima tamnu pozadinu na osnovu "udaljenosti od crne" u krugu boja.

Naši proizvodi daju sve od sebe u tome, međutim postoji skoro beskonačno boja u krugu boja i mogu postojati scenariji u kojima aplikacija bira da koristi tamni režim kada to nije prikladno, i obrnuto. Ova dokumentacija pokriva kako imati precizniju kontrolu nad tim.

#### Tehničke detalje

Otkrivamo tamni režim tako što prolazimo kroz elemente na stranici naviše od vidžeta za komentare, tražeći tamnu pozadinu kada se vidžet inicijalno učita.

Da biste prebacili tamni režim nakon ovog koraka, morate pozvati vidžet da ažurira svoju konfiguraciju. Ovo je pokriveno u odeljku `Ručna konfiguracija`.
