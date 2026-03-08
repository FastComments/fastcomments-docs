Vloge v Canvasu so med zagonom LTI samodejno preslikane v vloge FastComments. Ročna konfiguracija ni potrebna.

#### Mapiranje vlog

| Canvas Role | FastComments Role | Permissions |
|---|---|---|
| **Administrator** | Admin | Popoln dostop do računa, upravljanje vseh komentarjev in nastavitev |
| **Instructor** | Moderator | Urejanje in brisanje komentarjev, pripenjanje niti, upravljanje razprav |
| **Learner** | Commenter | Objavljanje komentarjev, odgovarjanje, glasovanje in uporaba omemb |

#### Kako deluje

Ko uporabnik zažene FastComments iz Canvasa, protokol LTI 1.3 vključuje njegovo vlogo v Canvasu. FastComments prebere to vlogo in samodejno dodeli ustrezna dovoljenja.

Če ima uporabnik več vlog (npr. Instructor, ki je tudi Admin), se uporabi vloga z najvišjimi pooblastili.

---