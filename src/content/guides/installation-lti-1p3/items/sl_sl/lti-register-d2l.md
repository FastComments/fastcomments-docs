D2L Brightspace omogoča dinamično registracijo prek skrbniškega vmesnika LTI Advantage. Potrebovali boste skrbniški dostop.

#### Odpiranje zaslona za registracijo

1. Prijavite se v svojo instanco Brightspace kot skrbnik.
2. Pomaknite se na **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Kliknite **Register Tool**. (Neposredni URL je `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Lepljenje URL-ja

Videli boste obrazec za registracijo. Ključno polje je **Tool initiation registration endpoint** (nekatere različice Brightspace ga označijo kot "Tool Initiation Registration URL").

Prilepite registracijski URL FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">pridobite ga tukaj</a>) v to polje. Ostala polja pustite prazna - samodejno jih izpolni FastComments med registracijskim rokovanjem.

Kliknite **Register**.

#### Odobritev orodja

Brightspace odpre pojavno okno, ki komunicira s FastComments, izmenja ključe in prikaže potrditveni zaslon. Pojavno okno se ob zaključku registracije samo zapre.

Novo orodje se prikaže v vašem seznamu orodij LTI Advantage. Privzeto Brightspace nova orodja označi kot **onemogočena** - preklopite stikalo na **omogočeno**, da ga bodo vaši tečaji lahko uporabljali.

#### Dodajanje namestitve

V Brightspace potrebujejo orodja LTI **namestitev (deployment)**, preden jih je mogoče uporabiti v tečajih:

1. Odprite na novo registrirano orodje FastComments.
2. Kliknite **View Deployments** > **New Deployment**.
3. Poimenujte namestitev (npr. "FastComments - All Courses"), izberite organizacijske enote, v katerih naj bo na voljo, in shranite.

Po prvem zagonu prek te namestitve FastComments pripne `deployment_id` na svoj konfiguracijski zapis - nadaljnji zagoni iz druge namestitve pod istim odjemalcem bodo zavrnjeni, razen če se ponovno registrirate.