AI agenti su dostupni na planovima **Flex** i **Pro**. Plan Creator ih ne uključuje.

### Ograničenja po planu

Svaka razina plana postavlja:

- **Zadane dnevne i mjesečne granice budžeta.** Možete ih smanjiti po agentu; podizanje granice po računu zahtijeva plan s višim limitom. Pogledajte [Budgets Overview](#budgets-overview).

Točni brojevi prikazani su na [stranica s cijenama](https://fastcomments.com/traffic-pricing) i na stranici naplate vašeg računa. Također su prikazani izravno u obrascu za uređivanje agenta, tako da ne morate napuštati obrazac da biste pronašli svoj limit.

FastComments Pro uključuje AI korištenje u vrijednosti od $200/mj. Flex se naplaćuje po cijeni od $20 po milijun tokena za sve modele (trenutno ili GLM 5.2 ili gpt-oss-120B).

### Naplata mora biti valjana

AI agenti rade samo kada najmodavac ima **valjanu naplatu u evidenciji**. Ako način plaćanja postane nevažeći, svi agenti se pauziraju, a stranica AI agenata prikazuje banner koji usmjerava osobu s ulogom **Billing Admin** da ažurira naplatu. Agenti se automatski nastavljaju kada se naplata obnovi – bez ponovnog pokretanja ili popunjavanja okidača koji su se aktivirali tijekom prekida.

Ovo je strogi preduvjet: potrošnja tokena naplaćuje se na vaš račun, pa platforma neće poslati nijedan LLM poziv bez ispravnog načina plaćanja.

### Tko može upravljati agentima

Stranice za upravljanje agentima zaštićene su ulogom **Customization Admin** na nadzornoj ploči. **Comment Moderator Admins** mogu pregledavati i odlučivati o odobrenjima (pogledajte [Approval Workflow](#approval-workflow)), ali ne mogu stvarati ili uređivati agente. **Billing Admins** primaju [e‑mailove upozorenja o budžetu](#budget-alerts) bez obzira imaju li pristup agentima.