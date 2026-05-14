D2L Brightspace omogoča dinamično registracijo prek administratorskega vmesnika LTI Advantage. Potrebovali boste administratorski dostop.

#### Open the Registration Screen

1. Prijavite se v svojo instanco Brightspace kot skrbnik.
2. Navigate to **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Kliknite **Register Tool**. (Neposredni URL je `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Paste the URL

Videli boste obrazec za registracijo. Ključno polje je **Tool initiation registration endpoint** (v nekaterih različicah Brightspace je označeno kot "Tool Initiation Registration URL").

Prilepite FastComments registracijski URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">dobite ga tukaj</a>) v to polje. Ostala polja pustite prazna - FastComments jih med postopkom registracije samodejno izpolni.

Kliknite **Register**.

#### Approve the Tool

Brightspace odpre pojavnega okna, ki komunicira s FastComments, izmenja ključe in prikaže potrditveni zaslon. Pojavno okno se samodejno zapre, ko je registracija dokončana.

Novo orodje se pojavi na seznamu orodij LTI Advantage. Privzeto Brightspace nova orodja označi kot **disabled** - preklopite stikalo na **enabled**, da ga bodo lahko uporabljali vaši tečaji.

#### Add a Deployment

V Brightspaceu LTI orodja potrebujejo **deployment** preden jih je mogoče uporabiti v predmetih:

1. Odprite novo registrirano FastComments orodje.
2. Click **View Deployments** > **New Deployment**.
3. Poimenujte deployment (npr. "FastComments - All Courses"), izberite organizacijske enote, v katerih naj bo na voljo, in shranite.

Po prvem zagonu prek tega deploymenta FastComments pritrdi `deployment_id` v svoj zapis konfiguracije - nadaljnji zagoni iz drugega deploymenta v okviru istega odjemalca bodo zavrnjeni, razen če se ponovno registrirate.