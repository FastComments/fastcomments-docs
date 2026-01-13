Nu gaan we uw aangepaste FastComments-code genereren. Gebruik de wizard hieronder om te configureren hoe u FastComments wilt laten werken op uw GoHighLevel-site:

[snippet id="gohighlevel-wizard"]

### Verschillende typen commentboxen

U kunt de regel `TYPE = 'commenting'` configureren om het gebruikte product te wijzigen (bijvoorbeeld kunt u het veranderen naar `live` voor streaming chat of `collab` voor collab chat).

### De commentbox waar u hem wilt plaatsen

Stel dat u commentboxen op specifieke delen van de pagina wilt plaatsen en niet op de standaardlocaties.
Change this line:

    const TARGET_ELEMENT_ID = ''; // instellen om target-divmodus te gebruiken

To:

    const TARGET_ELEMENT_ID = 'fc_box'; // instellen om target-divmodus te gebruiken

Then in the GHL editor, click the "code" button and add where you want the comments to go:

[inline-code-attrs-start title = 'GoHighLevel FastComments-div'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### Verschillend commentbox-type per pagina

Stel dat u wilt dat gebruikers stukken tekst markeren en bespreken, of in plaats daarvan de streaming chat UI gebruiken.

First follow the steps above in "De commentbox waar u hem wilt plaatsen".

Let op dat in dat kleine fragment `type="commenting"` staat.

Als u bijvoorbeeld collab chat wilt inschakelen, verander dan type naar `type="collab"`.

### Alleen tonen op specifieke pagina's

Als u `TARGET_ELEMENT_ID` niet instelt, kunt u in plaats daarvan de variabele `VALID_PATTERNS` configureren om in te stellen op welke URL-routes de opmerkingen moeten worden weergegeven. Standaard wordt het weergegeven op pagina's die `/post` in de URL bevatten.

### Collab Chat configureren

U kunt collab chat zo instellen dat het alleen samenwerkingsfunctionaliteit toevoegt rond HTML binnen een specifiek gebied. Stel bijvoorbeeld dat u de footer-code hierboven toevoegt en vervolgens deze div in de inhoud van de post/pagina plaatst om collab chat in te schakelen:

[inline-code-attrs-start title = 'Collab-chat met opgegeven inhoud'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

Dan zal het paragraafelement binnen de `<div>` collab chat ingeschakeld hebben, en niets anders op de pagina. Als u geen inhoud in de `<div>` plaatst, wordt collab chat ingeschakeld voor de gehele inhoud van de post.