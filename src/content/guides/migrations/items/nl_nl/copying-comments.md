In het geval dat gegevens verplaatst moeten worden, biedt FastComments een self‑service tool voor het verplaatsen van reacties tussen pagina’s en artikelen.

Zo ziet het formulier voor het kopiëren van reacties eruit:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; alt='Kopieercommentaarformulier met het From URL ID-veld en de To URL ID- en URL-velden'; title='Het kopieercommentaarformulier' app-screenshot-end]

### Invullen van de "From"-velden

Om te bepalen vanwaar reacties verplaatst moeten worden, moeten we simpelweg de bron `URL ID` weten.

Als je geen waarde voor `urlId` opgeeft in de configuratie van de reactiewidget, dan is dit een “schone” versie van de pagin URL.

Je kunt zien welke waarden je reacties hebben voor `URL ID` door ze te exporteren.

### Invullen van de "To"-velden

Om te bepalen waar reacties naartoe verplaatst moeten worden, moeten we de doel `URL ID` en `URL` weten.

De `URL ID` is de bucket waarin de reactie terechtkomt. Het `URL`‑veld wordt gebruikt zodat je rechtstreeks naar de reactie kunt navigeren vanuit e‑mails en moderatietools.

#### WordPress

Als je WordPress gebruikt, zou je bijvoorbeeld de artikel‑ID’s invoeren in de To/From `URL ID`‑velden in de migratietool, in plaats van een URL.