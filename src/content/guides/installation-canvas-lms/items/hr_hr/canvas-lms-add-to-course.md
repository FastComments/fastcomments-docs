#### Kako se komentari prikazuju u vašim tečajevima

Kada je LTI integracija omogućena i kada je instalirana vanjska aplikacija, FastComments radi automatski na temelju postavljanja koja ste konfigurirali:

#### Prikaz zadatka

Ako je omogućeno postavljanje **Prikaz zadatka**, komentari se automatski pojavljuju ispod svakog zadatka u tečaju. Studenti i nastavnici vide sekciju s nitima komentara kada pregledavaju zadatak — nije potrebna dodatna konfiguracija za svaki zadatak.

Svaki zadatak dobiva vlastitu zasebnu nit komentara.

#### Gumb uređivača za bogati sadržaj

Ako je omogućeno postavljanje **Gumb uređivača**, nastavnici mogu ugrađivati FastComments u bilo koji sadržaj koji koristi Rich Content Editor:

1. Uredite **Stranicu**, **Kviz** ili **Objavu**.
2. U alatnoj traci Rich Content Editora kliknite gumb **FastComments**.
3. FastComments se automatski ugrađuje u sadržaj.
4. Spremite stranicu.

Kada studenti pregledaju stranicu, ugrađeni FastComments widget se učitava s nitom komentara jedinstvenom za tu stranicu.

#### Automatski SSO

U oba postavljanja, studenti se automatski prijavljuju putem svog Canvas računa. Imena, e‑adrese i avatari sinkroniziraju se putem LTI pokretanja, nije potrebna zasebna prijava.

#### Zaključajte javni pristup (Preporučeno)

Prema zadanim postavkama, podaci komentara FastCommentsa su javno čitljivi. Bilo tko tko može pogoditi URL niti ili API endpoint može vidjeti njezine komentare, čak i izvan Canvasa. Za rasprave u tečajevima vrlo vjerojatno želite ograničiti pregled samo na upisane studente.

Open your <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">stranicu za prilagodbu widgeta</a> i kreirajte pravilo s omogućenom opcijom **Zahtijevaj SSO za prikaz komentara**, zatim postavite sigurnosnu razinu na **Sigurni SSO** tako da se niti mogu učitavati samo putem potpisanog LTI pokretanja.

Pogledajte [Zaštita niti komentara pomoću Single Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) za cjeloviti postupak, uključujući kako ograničiti pravilo na jednu domenu ili stranicu.