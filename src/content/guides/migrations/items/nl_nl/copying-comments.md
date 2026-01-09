In het geval dat gegevens verplaatst moeten worden, biedt FastComments een selfservice-tool voor het verplaatsen van reacties
tussen pagina's en artikelen.

Here's what the comment copy page form looks like:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; title='The Copy Comment Form' app-screenshot-end]

### 'From'-velden invullen

Om te bepalen waar reacties vandaan verplaatst moeten worden, moeten we gewoon de bron `URL ID` weten.

Als je geen waarde voor `urlId` doorgeeft in de configuratie van de reactiewidget, dan is dit een "schone" versie van de pagina-URL.

Je kunt zien welke waarden je reacties voor `URL ID` hebben door ze te exporteren.

### 'To'-velden invullen

Om te bepalen waar reacties naartoe verplaatst moeten worden, moeten we de doel-`URL ID` en `URL` weten.

De `URL ID` is de bucket waarin de reactie geplaatst wordt. Het `URL`-veld wordt gebruikt zodat je direct kunt navigeren
naar de reactie vanuit e-mails en moderatietools.

#### WordPress

Als je WordPress gebruikt, zou je bijvoorbeeld de Article IDs invullen in de To/From `URL ID` velden in de migratietool,
in plaats van een URL.