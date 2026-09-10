## Odpravljanje težav

**"You do not have permission" pri povezovanju.** Prijavljeni uporabnik ni API skrbnik na računu.  
Prosite lastnika računa, da dodeli API dovoljenje na strani Uporabniki, ali se povežite kot lastnik.

**Povezava je označena z napačnim spletnim mestom.** Stran za soglasje poveže račun, v katerega ste bili takrat prijavljeni.  
Prekinite povezavo v Zapierju, preklopite račune v nadzorni plošči FastComments in se ponovno povežite.

**Dogodki so prenehali priti.** Preverite stran Webhookov v nadzorni plošči. Naročnina, katere končna točka je šest dni zaporedno odpovedovala, je samodejno onemogočena in prikazuje razlog.  
Ponovno jo omogočite tam, ali izklopite in ponovno vklopite Zap. Če naročnina sploh manjka, jo je nekdo izbrisal; izklop in ponovni vklop Zapa jo ponovno ustvari.

**Zapier pravi, da je treba račun ponovno povezati.** Povezava je bila preklicana na strani Povezane aplikacije, uporabnik, ki jo je odobril, je izgubil API dovoljenje, ali je bil račun izbrisan.  
Ponovno povežite iz Zapierja.

**Dejanje spodleti z napako "does not have write access".** Povezava je bila odobrena le s pravicami za branje.  
Ponovno povežite in odobrite obe pravici.

**Omejitve hitrosti in krediti.** Dejanja in iskanja porabijo API kredite iz vašega paketa in so podvržena istim omejitvam hitrosti kot REST API. Sprožilci ne porabijo nobenega. Zap, ki naleti na omejitev, Zapier ponovi po zakasnitvi, ki jo FastComments poroča.

**Spustni seznam Domene je prazen.** Domene se pojavijo, ko so nastavljene na strani Domene v nadzorni plošči FastComments.  
Pustite polje prazno, da prejemate dogodke za vsako domeno.