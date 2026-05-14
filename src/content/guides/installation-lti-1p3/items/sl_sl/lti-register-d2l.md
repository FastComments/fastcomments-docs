D2L Brightspace omogoča dinamično registracijo prek skrbniškega vmesnika LTI Advantage. Potrebovali boste skrbniški dostop.

#### Open the Registration Screen

1. Prijavite se v svojo instanco Brightspace kot skrbnik.
2. Pojdite na **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Kliknite **Register Tool**. (Neposredni URL je `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Paste the URL

Videli boste obrazec za registracijo. Ključno polje je **Tool initiation registration endpoint** (nekatere različice Brightspace-a ga označujejo kot "Tool Initiation Registration URL").

Prilepite FastComments registracijski URL v to polje. Druga polja pustite prazna - med prijazom za registracijo jih FastComments samodejno zapolni.

Kliknite **Register**.

#### Approve the Tool

Brightspace odpre pojavno okno, ki komunicira s FastComments, izmenja ključe in prikaže zaslon s potrditvijo. Pojavno okno se samodejno zapre, ko je registracija končana.

Novo orodje se prikaže na seznamu vaših orodij LTI Advantage. Privzeto Brightspace nova orodja označi kot **disabled** - preklopite stikalo na **enabled**, da ga lahko uporabljajo vaši tečaji.

#### Add a Deployment

V Brightspace morajo imeti LTI orodja pred uporabo v tečajih dodeljeno **deployment**:

1. Odprite novo registrirano orodje FastComments.
2. Kliknite **View Deployments** > **New Deployment**.
3. Poimenujte deployment (npr. "FastComments - Vsi tečaji"), izberite org enote, v katerih naj bo na voljo, in shranite.

Po prvem zagonu prek tega deploymenta FastComments pripne `deployment_id` k svojemu zapisu konfiguracije - nadaljnji zagoni iz drugega deploymenta pod istim odjemalcem bodo zavrnjeni, če se ne ponovno registrirate.