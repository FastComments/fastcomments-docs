[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Som standard vil FastComments gengive afstemningsmuligheder som op- og ned-pile, så brugerne kan enten op- eller nedstemme en kommentar.

Det er dog muligt at ændre stilen på afstemningsværktøjslinjen. De aktuelle muligheder er standard Op/Ned-knapperne eller at bruge en hjerte‑stil afstemningsmekanisme.

Vi bruger **voteStyle**‑flaget som følger:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Aktiver hjerteknap'; code-example-end]

Vi anbefaler kraftigt, at du gør dette uden kode, da det også aktiverer server‑side valideringer. På widget‑tilpasningssiden, se sektionen "Vote Style".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; alt='Afstemningsstil-indstilling på widget‑tilpasningssiden, der tilbyder op- og ned-pile eller hjerteafstemning'; title='Skift afstemningsstil' app-screenshot-end]

Afstemning kan også deaktiveres, se `Disable Voting` over stilindstillingerne.

---