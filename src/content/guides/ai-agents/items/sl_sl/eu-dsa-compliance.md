FastComments izvaja 17. člen Uredbe EU o digitalnih storitvah (DSA) za najemnike v regiji EU: **popolnoma avtomatizirane suspenzije uporabnikov niso dovoljene**.

### Kaj to pomeni v praksi

Ko je vaš najemnik v regiji EU, na obrazcu za urejanje agenta:

- Potrditveno polje **Approvals** za `ban_user` je **zaklenjeno v položaju vklopljeno** in ga ni mogoče odkljukati.
- Nalepka pravi: "EU DSA Article 17: user suspensions require human review. 'Ban a user' is locked on and cannot be fully automated in the EU region."
- Orodna namig na stolpcu za odobritev pravi: "Locked on by EU DSA Article 17 - fully-automated bans are not permitted in the EU region."

Ne glede na to, kako drugače konfigurirate, vsak klic `ban_user` s strani kateregakoli agenta na najemniku v regiji EU pride v [approvals inbox](#approval-workflow) v namen človeškega pregleda. Prepoved se ne uveljavi, dokler je ne odobri človek.

### Zakaj se to izvaja na ravni platforme, ne na ravni poziva

Sistemske pozive lahko model zlorabi ali jih ignorira. Usklajenost s 17. členom je preveč pomembna, da bi jo zaupali dobremu vedenju modela; mora biti stroga strežniška kontrola, ki jo izvaja sam dispečer orodij. In to tudi počnemo.

### Kaj gre in kaj ne gre skozi odobritev

- **`ban_user`**: vedno zablokiran v EU. Vključuje:
  - Vidne prepovedi (`shadowBan: false`).
  - Shadow prepovedi (`shadowBan: true`).
  - Prepovedi z `deleteAllUsersComments: true`.
  - Prepovedi z `banIP: true`.
- Vse variante prepovedi prispejo v inbox za odobritve z razlogovanjem in stopnjo zaupanja agenta; človek jih odobri ali zavrne.

Ostala agentova orodja (`mark_comment_spam`, `warn_user`, `lock_comment`, itd.) NAJ niso prizadeta od 17. člena. Še vedno jih lahko avtomatizirate. 17. člen se nanaša izrecno na suspenzije uporabnikov.

### Kaj pa najemniki zunaj EU

Zaklep se ne uporablja izven regije EU. Kljub temu lahko izberete, da `ban_user` vseeno zahtevate preko odobritve - to močno priporočamo v prvih tednih delovanja kateregakoli moderacijskega agenta - vendar to ni obvezno.

### Shadow prepovedi

Shadow prepovedi se za namene 17. člena štejejo kot suspenzije (uporabnik lahko objavlja, vendar je njihova vsebina skrita). Urejene so enako kot vidne prepovedi.

### Ugotavljanje regije

Regijo določa na ravni procesa okoljska spremenljivka `REGION` na namestitvi FastComments (prebrano z `isEURegion()` v `models/constants.ts`). Ni polja regije na ravni najemnika - zaklep velja za vsak najemnik na instanci nameščeni v EU. Če preselite svoje podatke z namestitve izven EU na namestitev v EU, zaklep začne veljati za vse najemnike na tej instanci.

### Kaj, če vsi recenzenti niso na voljo

Odobritev bo ostala v inboxu, dokler se ne odloči. Samodejno poteče 90 dni po ustvarjanju. Ni poti "ni recenzenta na voljo, preiti na avtomatizirano odločitev" — to bi nasprotovalo namenu 17. člena.

Če je vaša skupnost tako obsežna, da EU prepovedi ni mogoče pregledati v razumljivem času, razmislite o:

- Dodajanju več recenzentov (glejte [Approval Notifications](#approval-notifications)).
- Preklopu agenta na bolj agresivno uporabo [`warn_user`](#tool-warn-user), saj opozorila niso predmet 17. člena.
- Zmanjšanju nagnjenosti agenta k prepovedim z zaostritvijo [community guidelines](#community-guidelines) ali [initial prompt](#personality-prompt).

### Glej tudi

- [Tool: ban_user](#tool-ban-user) za to, kaj počne `ban_user` in uničujoče možnosti za dodatne privolitve.
- [Approval Workflow](#approval-workflow) za celotno življenjsko dobo odobritve.