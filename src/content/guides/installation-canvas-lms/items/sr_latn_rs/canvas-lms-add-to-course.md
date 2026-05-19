#### Kako se komentari pojavljuju u vašim kursevima

Kada je LTI integracija omogućena i External App instalirana, FastComments radi automatski na osnovu postavki za prikaz (placements) koje ste konfigurisali:

#### Assignment View

Ako je postavka **Assignment View** omogućena, komentari se automatski pojavljuju ispod svakog zadatka u kursu. Studenti i instruktori vide ugnježdeni odeljak za komentare kada pregledaju zadatak — nije potrebna dodatna konfiguracija po zadatku.

Svaki zadatak ima svoju posebnu nit komentara.

#### Rich Content Editor Button

Ako je postavka **Editor Button** omogućena, instruktori mogu da ugrađuju FastComments u bilo koji sadržaj koji koristi Rich Content Editor:

1. Izmenite **Page**, **Quiz**, ili **Announcement**.
2. U traci sa alatkama Rich Content Editora kliknite na dugme **FastComments**.
3. FastComments se automatski ugrađuje u sadržaj.
4. Sačuvajte stranicu.

Kada studenti pregledaju stranicu, ugrađeni FastComments vidžet se učitava sa nitom komentara koja je jedinstvena za tu stranicu.

#### Automatic SSO

U obe lokacije za prikaz, studenti se automatski prijavljuju putem svog Canvas naloga. Imena, adrese e-pošte i avatari se sinhronizuju preko LTI launch, nije potrebna zasebna prijava.

#### Lock Down Public Access (Recommended)

Po defaultu, podaci komentara FastComments-a su javno čitljivi. Bilo ko ko može pogoditi URL niti ili API endpoint može videti njene komentare, čak i van Canvas-a. Za diskusije u okviru kursa gotovo sigurno želite ograničiti pregled samo na upisane studente.

Otvorite svoju <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">stranicu za prilagođavanje vidžeta</a> i kreirajte pravilo sa omogućenim **Require SSO To View Comments**, zatim postavite nivo bezbednosti na **Secure SSO** tako da se niti mogu učitati samo kroz potpisano LTI pokretanje.

Pogledajte [Protecting Comment Threads With Single-Sign-On](/guide-customizations-and-configuration.html#sso-require-to-view-comments) za kompletan vodič, uključujući kako da ograničite pravilo na jedinstveni domen ili stranicu.