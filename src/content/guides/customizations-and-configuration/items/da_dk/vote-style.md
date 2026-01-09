[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

Som standard viser FastComments stemmeindstillinger som op- og ned-knapper, så brugere kan stemme en kommentar op eller ned.

Det er dog muligt at ændre stilen på stemmeværktøjslinjen. De nuværende muligheder er standard op-/ned-knapperne eller at bruge en hjertebaseret stemmefunktion.

Vi bruger **voteStyle**-flaget som følger:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

Vi anbefaler kraftigt, at du gør dette uden kode, da det også aktiverer validering på serversiden. På siden til tilpasning af widgeten, se sektionen "Vote Style".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

Stemmegivning kan også deaktiveres, se `Disable Voting` ovenfor ved stilindstillingerne.