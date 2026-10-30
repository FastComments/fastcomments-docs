AI agenti so na voljo v načrtih **Flex** in **Pro**. Načrt Creator jih ne vključuje.

### Omejitve na ravni načrta

- **Privzete dnevne in mesečne omejitve proračuna.** Te lahko znižate na ravni agenta; zvišanje omejitve na ravni računa zahteva načrt z višjim limitom. Glejte [Pregled proračunov](#budgets-overview).

Natančne številke so prikazane na [strani s cenami](https://fastcomments.com/traffic-pricing) in na strani za obračun vašega računa. Prav tako so prikazane neposredno v obrazcu za urejanje agenta, tako da vam nikoli ni treba zapustiti obrazca, da bi našli svojo omejitev.

FastComments Pro vključuje uporabo AI v vrednosti 200 $/mesec. Flex se obračunava po ceni 20 $ na milijon žetonov za vse modele (trenutno bodisi GLM 5.2 ali gpt-oss-120B).

### Plačilo mora biti veljavno

AI agenti se izvajajo le, ko ima najemnik **veljavno plačilo v evidenci**. Če postane način plačila neveljaven, se vsi agenti zaustavijo, stran AI Agentov prikaže pasico, ki usmerja osebo z vlogo **Billing Admin** k posodobitvi plačila. Agent se samodejno nadaljuje, ko se plačilo obnovi – brez ponovnega izvajanja ali dopolnjevanja sprožilcev, ki so se sprožili med izpadom.

To je stroga predpogoj: poraba žetonov se zaračuna na vaš račun, zato platforma ne bo izvedla nobenega klica LLM brez delujočega načina plačila.

### Kdo lahko upravlja agente

Strani za upravljanje agentov so zaščitene z vlogo **Customization Admin** v nadzorni plošči. **Upravitelji komentarjev Moderatorji** lahko pregledajo in odločajo o odobritvah (glejte [Potek odobritve](#approval-workflow)), vendar ne morejo ustvarjati ali urejati agentov. **Upravitelji plačil** prejemajo [e‑mail obvestila o proračunu](#budget-alerts), ne glede na to, ali imajo dostop do agentov.