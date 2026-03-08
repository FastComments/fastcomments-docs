Canvas uloge se automatski mapiraju na FastComments uloge tokom LTI pokretanja. Nije potrebna ručna konfiguracija.

#### Mapiranje uloga

| Canvas Role | FastComments Role | Dozvole |
|---|---|---|
| **Administrator** | Admin | Puni pristup nalogu, upravljanje svim komentarima i podešavanjima |
| **Instructor** | Moderator | Uređivanje i brisanje komentara, prikvačivanje niti, upravljanje diskusijama |
| **Learner** | Commenter | Objavljivanje komentara, odgovaranje, glasanje i korišćenje pomena |

#### Kako to funkcioniše

Kada korisnik pokrene FastComments iz Canvas-a, LTI 1.3 protokol uključuje njegovu Canvas ulogu. FastComments čita tu ulogu i automatski dodeljuje odgovarajuće dozvole.

Ako korisnik ima više uloga (npr. an Instructor koji je takođe an Admin), koristi se uloga sa najvećim privilegijama.

---