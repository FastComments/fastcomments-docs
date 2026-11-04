Nu gaan we je aangepaste FastComments-code genereren. Gebruik de wizard hieronder om te configureren hoe je wilt dat FastComments werkt op je GoHighLevel-site:

[snippet id="gohighlevel-wizard"]

### Verschillende soorten commentaarvakken

Je kunt de regel `TYPE = 'commenting'` aanpassen om het gebruikte product te wijzigen (je kunt deze bijvoorbeeld veranderen in `live` voor streaming chat of `collab` voor collab chat).

### Het commentaarvak plaatsen waar jij het wilt

Stel dat je commentaarvakken op specifieke delen van de pagina wilt plaatsen en niet op de standaardlocaties.
Wijzig deze regel:

    const TARGET_ELEMENT_ID = ''; // set to use target div mode

Naar:

    const TARGET_ELEMENT_ID = 'fc_box'; // set to use target div mode

Klik vervolgens in de GHL-editor op de knop "code" en voeg toe waar je de reacties wilt hebben:

[inline-code-attrs-start title = 'GoHighLevel FastComments Div'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### Verschillend type commentaarvak per pagina

Stel dat je wilt dat gebruikers stukken tekst kunnen markeren en bespreken, of dat je de streaming chat-UI wilt gebruiken.

Volg eerst de stappen hierboven in "Het commentaarvak plaatsen waar jij het wilt".

Let op: in dat kleine snippet staat `type="commenting"`.

Als je bijvoorbeeld collab chat wilt inschakelen, wijzig dan het type in `type="collab"`.

### Alleen weergeven op specifieke pagina's

Als je `TARGET_ELEMENT_ID` niet instelt, kun je in plaats daarvan de variabele `VALID_PATTERNS` configureren om in te stellen op welke URL-routes de reacties moeten worden weergegeven. Standaard worden ze
weergegeven op pagina's die `/post` in de URL bevatten.

### Collab chat configureren

Je kunt collab chat opdracht geven om alleen collaboratieve functionaliteit toe te voegen rond HTML binnen een specifiek gebied. Stel bijvoorbeeld dat je
de footercode hierboven toevoegt en vervolgens deze div toevoegt in de inhoud van de post/pagina om collab chat in te schakelen:

[inline-code-attrs-start title = 'Collab Chat Met Gespecificeerde Inhoud'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>Deze inhoud krijgt collab chat!</p></div>
[inline-code-end]

Dan heeft het alinea-element binnen de `<div>` collab chat ingeschakeld, en niets anders op de pagina. Als je
geen inhoud in de `<div>` plaatst, wordt collab chat ingeschakeld op de gehele posttekst.