AI Agents su dostupni na planovima **Flex** i **Pro**. Plan Creator ih ne uključuje.

### Plan-level limits

Svaki nivo plana postavlja:

- **Podrazumevane dnevne i mesečne granice budžeta.** Možete ih smanjiti po agentu; za povećanje granice po nalogu potreban je plan sa višim limitom. Pogledajte [Pregled budžeta](#budgets-overview).

Tačne vrednosti se prikazuju na [pricing page](https://fastcomments.com/traffic-pricing) i na stranici za naplatu vašeg naloga. Takođe se prikazuju direktno u formi za uređivanje agenta, tako da nikad ne morate napustiti formu da biste pronašli svoj limit.

FastComments Pro uključuje $200/mesečno vrednosti AI upotrebe. Flex se naplaćuje po stopi od $20 za milion tokena za sve modele (trenutno GLM 5.1 ili gpt-oss-120B-turbo).

### Billing must be valid

AI Agents se pokreću samo ako zakupac ima **važeću naplatu u evidenciji**. Ako način plaćanja postane nevažeći, svi agenti se pauziraju i stranica AI Agents prikazuje baner koji upućuje osobu sa ulogom **Billing Admin** da ažurira naplatu. Agenti se automatski nastave kada se naplata obnovi - nema ponovnog pokretanja niti popunjavanja (backfill) okidača koji su se aktivirali tokom prekida.

Ovo je strogi preduslov: troškovi tokena se fakturišu na vaš nalog, pa platforma neće poslati nijedan LLM poziv bez ispravne metode plaćanja.

### Who can manage agents

Stranice za administraciju agenata dostupne su samo korisnicima sa ulogom na kontrolnoj tabli **Customization Admin**. **Comment Moderator Admins** mogu pregledati i donositi odluke o odobravanjima (pogledajte [Proces odobravanja](#approval-workflow)) ali ne mogu kreirati ili uređivati agente. **Billing Admins** primaju [e-poruke upozorenja o budžetu](#budget-alerts) bez obzira na to da li imaju pristup agentima.