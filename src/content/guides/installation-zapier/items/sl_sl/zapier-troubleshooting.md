## Odpravljanje težav

**"Nimate dovoljenja" pri povezovanju.** Prijavljeni uporabnik ni skrbnik API-ja na računu.
Prosite lastnika računa, da odobri dovoljenje za API na strani Users, ali pa se povežite kot lastnik.

**Povezava je označena z napačnim mestom.** Stran s soglasjem poveže račun, v katerega ste bili
prijavljeni v tistem trenutku. Prekinite povezavo v Zapier, preklopite račune na nadzorni plošči FastComments in se povežite znova.

**Dogodki so prenehali prihajati.** Preverite stran Webhooks na nadzorni plošči. Naročnina, katere končna točka je
šest dni zapored neuspešno odgovarjala, se samodejno onemogoči in prikaže razlog. Tam jo znova omogočite, ali pa Zap izklopite in ponovno vklopite.
Če naročnina sploh manjka, jo je nekdo izbrisal; izklop in ponovni vklop Zapa jo ponovno ustvari.

**Zapier pravi, da je treba račun znova povezati.** Povezava je bila preklicana na strani Connected Apps,
uporabnik, ki jo je odobril, je izgubil dovoljenje za API, ali pa je bil račun izbrisan. Znova se povežite iz Zapierja.

**Dejanje ne uspe z napako "does not have write access".** Povezava je bila odobrena samo z dovoljenjem za branje.
Znova se povežite in odobrite obe dovoljenji.

**Omejitve hitrosti in krediti.** Dejanja in iskanja porabijo API kredite iz vašega paketa in so podvržena
enakim omejitvam hitrosti kot REST API. Sprožilci ne porabijo ničesar. Zap, ki naleti na omejitev, poskusi Zapier
znova po zamudi, ki jo sporoči FastComments.

**Spustni seznam Domain je prazen.** Domene se prikažejo, ko so nastavljene na strani Domains na
nadzorni plošči FastComments. Pustite polje prazno, če želite prejemati dogodke za vse domene.