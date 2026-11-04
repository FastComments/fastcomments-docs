## Povežite svoj račun

1. V Zapierju dodajte korak FastComments v Zap ali odprite stran aplikacije FastComments v Zapier App Directory.
2. Izberite **Sign in to FastComments**. Zapier najprej vpraša za vašo regijo: izberite **United States**, razen če je bil
   vaš račun ustvarjen v regiji EU (`eu.fastcomments.com`).
3. Odpre se okno FastComments. Prijavite se, če še niste prijavljeni.
4. Preglejte stran s soglasjem. Prikazuje aplikacijo Zapier, račun, s katerim bo povezana, in zahtevana
   dovoljenja (branje in pisanje). Izberite **Approve**.
5. Zapier shrani povezavo in jo označi z imenom vašega spletnega mesta ter uporabniškim imenom.

Povezava uporablja OAuth. V Zapier se ne kopira noben API ključ, žeton, ki ga ima Zapier, pa deluje samo za
račun, ki ste ga odobrili.

## Kdo lahko poveže

Oseba, ki odobrava povezavo, mora biti **API admin** na računu FastComments. Lastniki računov
imajo to dovoljenje; drugim članom ekipe ga je mogoče dodeliti na strani Users. Nekdo brez tega dovoljenja namesto
obrazca za soglasje vidi stran »nimate dovoljenja«.

## Povezovanje pravega spletnega mesta

Stran s soglasjem poveže račun, v katerega ste trenutno prijavljeni. Če upravljate več računov,
pred odobritvijo preklopite na pravega iz preklopnika računov ali uporabite povezavo **switch account** na
strani s soglasjem. Oznaka povezave v Zapierju prikazuje ime spletnega mesta, zato je napačna izbira enostavno opaziti.

## Pregled in preklic dostopa

Vsaka povezava se prikaže pod **Connected Apps** v nadzorni plošči FastComments, skupaj z dovoljenji, ki jih
ima, in časom zadnje uporabe. Če jo tam prekličete, se Zapier takoj odklopi; vsak Zap, ki uporablja to
povezavo, se ustavi, dokler ni ponovno povezan. Povezavo lahko odstranite tudi s strani Zapierja pod
**My Apps**.