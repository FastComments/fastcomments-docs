AI agenti su dostupni na planovima **Flex** i **Pro**. Plan Creator ih ne uključuje.

### Ograničenja na nivou plana

- **Podrazumevana dnevna i mesečna ograničenja budžeta.** Možete ih smanjiti po agentu; podizanje ograničenja po nalogu zahteva plan sa višim plafonom. Pogledajte [Budgets Overview](#budgets-overview).

Tačni brojevi su prikazani na [stranica sa cenama](https://fastcomments.com/traffic-pricing) i na stranici za naplatu vašeg naloga. Takođe su prikazani direktno u formi za uređivanje agenta, tako da nikada ne morate napustiti formu da biste pronašli svoje ograničenje.

FastComments Pro uključuje AI korišćenje u vrednosti od $200 mesečno. Flex se naplaćuje po stopi od $20 po milionu tokena za sve modele (trenutno ili GLM 5.2 ili gpt-oss-120B).

### Naplata mora biti važeća

AI agenti se pokreću samo kada zakupac ima **važeću naplatu u evidenciji**. Ako metoda plaćanja postane nevažeća, svi agenti se pauziraju i stranica AI Agenti prikazuje baner koji usmerava osobu koja ima ulogu **Billing Admin** da ažurira naplatu. Agenti se automatski nastavljaju kada naplata bude obnovljena – bez ponovnog pokretanja ili dopunjavanja okidača koji su se aktivirali tokom prekida.

Ovo je strogi uslov: potrošnja tokena se naplaćuje na vaš nalog, tako da platforma neće poslati nijedan LLM poziv bez radne metode plaćanja.

### Ko može upravljati agentima

Stranice za administraciju agenata su zaštićene ulogom **Customization Admin** na kontrolnoj tabli. **Comment Moderator Admins** mogu pregledati i odlučivati o odobrenjima (pogledajte [Approval Workflow](#approval-workflow)), ali ne mogu kreirati ili uređivati agente. **Billing Admins** primaju [emailove za upozorenje o budžetu](#budget-alerts) bez obzira da li imaju pristup agentima.