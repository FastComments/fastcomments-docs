AI Agents so na voljo v načrtih **Flex** in **Pro**. Načrt Creator jih ne vključuje.

### Omejitve na ravni načrta

Vsaka raven načrta določa:

- **Privzete dnevne in mesečne omejitve proračuna.** Te lahko znižate za posameznega agenta; dvig zgornje meje za račun zahteva načrt z višjo omejitvijo. Oglejte si [Pregled proračunov](#budgets-overview).

Točne številke so prikazane na [strani s cenami](https://fastcomments.com/traffic-pricing) in na strani z obračunavanjem vašega računa. Prikazane so tudi neposredno v obrazcu za urejanje agenta, tako da vam ni treba zapustiti obrazca, da bi našli svojo omejitev.

FastComments Pro vključuje $200/mo vrednega AI-porabe. Načrt Flex se zaračunava po stopnji $20 na milijon tokenov za vse modele (trenutno GLM 5.1 ali gpt-oss-120B-turbo).

### Plačilni podatki morajo biti veljavni

AI Agents se izvajajo samo, kadar ima tenant **veljavne podatke za obračun**. Če plačilna metoda postane neveljavna, se vsi agenti začasno ustavijo in stran AI Agents prikaže pasico, ki osebo z vlogo **Billing Admin** usmeri k posodobitvi plačilnih podatkov. Agenti se samodejno nadaljujejo, ko so plačilni podatki ponovno vzpostavljeni — brez ponovnega izvajanja ali dopolnitve sprožilcev, ki so se sprožili med izpadom.

To je strogi predpogoj: poraba tokenov se zaračuna vašemu računu, zato platforma ne bo poslala nobenega LLM klica brez delujoče plačilne metode.

### Kdo lahko upravlja agente

Strani za upravljanje agentov so omejene z vlogo nadzorne plošče **Customization Admin**. Uporabniki z vlogo **Comment Moderator Admins** lahko pregledajo in odločajo o odobritvah (glejte [Potek odobritve](#approval-workflow)), vendar ne morejo ustvarjati ali urejati agentov. Uporabniki z vlogo **Billing Admins** prejemajo [e-poštna obvestila o proračunu](#budget-alerts), ne glede na to, ali imajo dostop do agentov.