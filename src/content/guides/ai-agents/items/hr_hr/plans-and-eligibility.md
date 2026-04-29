AI Agenti su dostupni na planovima **Flex** i **Pro**. Plan **Creator** ih ne uključuje.

### Ograničenja na razini plana

Svaka razina plana postavlja:

- **Zadane dnevne i mjesečne granice budžeta.** Možete ih sniziti po agentu; za povećanje granice po računu potreban je plan s višim limitom. Vidi [Pregled budžeta](#budgets-overview).

Točne vrijednosti prikazane su na [stranici s cijenama](https://fastcomments.com/traffic-pricing) i na stranici za naplatu vašeg računa. Također su prikazane izravno u obrascu za uređivanje agenta kako ne biste morali napuštati obrazac da biste pronašli svoju granicu.

FastComments Pro uključuje $200/mo vrijednosti AI korištenja. Flex se naplaćuje po stopi od $20 po milijun tokena za sve modele (trenutno ili GLM 5.1 ili gpt-oss-120B-turbo).

### Podaci o naplati moraju biti valjani

AI Agenti se pokreću samo ako račun ima **valjane podatke o naplati**. Ako metoda plaćanja postane nevaljana, svi agenti se pauziraju i stranica AI Agents prikazuje banner koji upućuje onoga tko ima ulogu **Billing Admin** da ažurira podatke o naplati. Agenti se automatski nastavljaju kada se naplata obnovi — bez ponovnog pokretanja ili naknadnog popunjavanja okidača koji su se aktivirali tijekom prekida.

Ovo je čvrst preduvjet: trošak tokena naplaćuje se na vaš račun, pa platforma neće poslati nijedan LLM poziv bez ispravne metode plaćanja.

### Tko može upravljati agentima

Stranice za administraciju agenata zaštićene su ulogom nadzorne ploče **Customization Admin**. **Comment Moderator Admins** mogu pregledavati i odlučivati o odobrenjima (vidi [Tijek odobravanja](#approval-workflow)) ali ne mogu stvarati niti uređivati agente. **Billing Admins** primaju [e-poruke o upozorenju na budžet](#budget-alerts) bez obzira na to imaju li pristup agentima.