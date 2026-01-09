Nu vil vi generere din tilpassede FastComments-kode. Brug guiden nedenfor til at konfigurere, hvordan du vil have FastComments til at fungere på dit GoHighLevel-site:

[snippet id="gohighlevel-wizard"]

### Forskellige typer kommentarbokse

Du kan konfigurere linjen `TYPE = 'commenting'` for at skifte det anvendte produkt (for eksempel kan du ændre den til `live` for streamingchat eller `collab` for collab-chat).

### Placering af kommentarboksen hvor du vil

Lad os sige, at du vil placere kommentarbokse på bestemte dele af siden og ikke på standardplaceringerne.
Skift denne linje:

    const TARGET_ELEMENT_ID = ''; // angiv for at bruge target-div-tilstand

Til:

    const TARGET_ELEMENT_ID = 'fc_box'; // angiv for at bruge target-div-tilstand

Derefter skal du i GHL-editoren klikke på "code"-knappen og tilføje, hvor du vil have kommentarerne skal vises:

[inline-code-attrs-start title = 'GoHighLevel FastComments-div'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### Forskellig kommentarboks-type pr. side

Lad os sige, at du vil lade brugere fremhæve og diskutere tekststykker eller i stedet bruge streamingchat-brugerfladen.

Følg først trinnene ovenfor under "Placering af kommentarboksen hvor du vil".

Bemærk, at der i den lille kodeudsnit står `type="commenting"`.

Hvis du for eksempel vil aktivere collab-chat, skal du ændre typen til `type="collab"`.

### Vis kun på bestemte sider

Hvis du ikke sætter `TARGET_ELEMENT_ID`, kan du i stedet konfigurere variablen `VALID_PATTERNS` for at angive, hvilke URL-ruter kommentarerne skal vises på. Som standard vises de på sider, der indeholder `/post` i URL'en.

### Konfiguration af collab-chat

Du kan få collab-chat til kun at tilføje kollaborativ funktionalitet omkring HTML inden for et bestemt område. For eksempel, hvis du tilføjer footer-koden ovenfor og derefter indsætter denne div i post-/sideindholdet for at aktivere collab-chat:

[inline-code-attrs-start title = 'Collab-chat med angivet indhold'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

Så vil paragraf-elementet inde i `<div>` have collab-chat aktiveret, og intet andet på siden. Hvis du ikke lægger noget indhold i `<div>`, vil det aktivere collab-chat for hele indlæggets indhold.