Uloge u Canvasu se automatski mapiraju na uloge u FastComments tokom LTI pokretanja. Nije potrebna ručna konfiguracija.

#### Mapiranje uloga

| Uloga u Canvasu | Uloga u FastComments | Dozvole |
|---|---|---|
| **Administrator** | Admin | Puni pristup nalogu, upravljanje svim komentarima i postavkama |
| **Instructor** | Moderator | Uređivanje i brisanje komentara, prikvačivanje niti, upravljanje diskusijama |
| **Learner** | Commenter | Objavljivanje komentara, odgovaranje, glasanje i korišćenje pominjanja |

#### Kako radi

Kada korisnik pokrene FastComments iz Canvasa, LTI 1.3 protokol uključuje njegovu ulogu u Canvasu. FastComments čita tu ulogu i automatski dodeljuje odgovarajuće dozvole.

Ako korisnik ima više uloga (npr. nastavnik koji je takođe Admin), koristi se uloga sa najvećim privilegijama.

---