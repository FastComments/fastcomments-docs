Canvas uloge se automatski mapiraju na FastComments uloge tijekom LTI pokretanja. Ručna konfiguracija nije potrebna.

#### Mapiranje uloga

| Canvas Role | FastComments Role | Permissions |
|---|---|---|
| **Administrator** | Admin | Potpun pristup računu, upravljanje svim komentarima i postavkama |
| **Instructor** | Moderator | Uređivanje i brisanje komentara, pričvršćivanje niti, upravljanje raspravama |
| **Learner** | Commenter | Objavljivanje komentara, odgovaranje, glasanje i korištenje spominjanja |

#### Kako to funkcionira

Kada korisnik pokrene FastComments iz Canvasa, LTI 1.3 protokol uključuje njegovu Canvas ulogu. FastComments čita tu ulogu i automatski dodjeljuje odgovarajuće dozvole.

Ako korisnik ima više uloga (npr. an Instructor koji je također Admin), koristi se uloga s najvećim privilegijama.

---