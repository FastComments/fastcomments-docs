#### Kako se komentari pojavljuju u vašim kursevima

Kada se LTI integracija omogući i eksterna aplikacija instalira, FastComments radi automatski na osnovu plasmana koje ste konfigurisali:

#### Pregled zadatka

Ako je omogućen plasman **Assignment View**, komentari se automatski pojavljuju ispod svakog zadatka u kursu. Studenti i instruktori vide sekciju sa nitima komentara kada pregledaju zadatak — nije potrebna dodatna podešavanja za svaki zadatak.

Svaki zadatak dobija svoju odvojenu nit komentara.

#### Dugme u Rich Content Editor

Ako je omogućen plasman **Editor Button**, instruktori mogu da ugrade FastComments u bilo koji sadržaj koji koristi Rich Content Editor:

1. Izmenite **Stranicu**, **Kviz** ili **Objavu**.
2. U traci alata Rich Content Editor kliknite na dugme **FastComments**.
3. FastComments se automatski ugrađuje u sadržaj.
4. Sačuvajte stranicu.

Kada studenti pregledaju stranicu, ugrađeni FastComments vidžet se učitava sa niti komentara jedinstvenom za tu stranicu.

#### Automatski SSO

U oba plasmana, studenti se automatski prijavljuju preko svog Canvas naloga. Imena, imejlovi i avatari se sinhronizuju kroz LTI pokretanje, nije potreban odvojeni login.

#### Ograničite javni pristup (preporučeno)

Podrazumevano, podaci o komentarima u FastComments su javno čitljivi. Svako ko pogodi URL niti ili API endpoint može da vidi komentare, čak i van Canvas-a. Za diskusije u kursu gotovo sigurno želite da ograničite pregledanje samo na upisane studente.

Otvorite svoju <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">stranicu za prilagođavanje vidžeta</a> i kreirajte pravilo sa omogućenom opcijom **Require SSO To View Comments**, a zatim postavite bezbednosni nivo na **Secure SSO** tako da niti mogu da se učitaju samo kroz potpisano LTI pokretanje.

Pogledajte [Zaštita niti komentara pomoću Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) za kompletan vodič, uključujući kako da ograničite pravilo na jedan domen ili stranicu.