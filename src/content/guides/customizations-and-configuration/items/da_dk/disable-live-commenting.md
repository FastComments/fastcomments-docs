---
[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Som standard vil FastComments have live-kommentering aktiveret.

Det betyder, at hver bruger af kommentarthreaden skal se det samme indhold.

For eksempel, hvis en kommentar tilføjes, skal den vises. Hvis en kommentar redigeres eller fjernes,
så vil de kommentarer blive redigeret eller fjernet for alle brugere af tråden. Det samme gælder for stemmer og alle moderationshandlinger.

Vi kan dog deaktivere dette:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Dette kan også gøres uden kode. På widget-tilpasningssiden, se sektionen "Disable Live Commenting".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; alt='Deaktiver Live Kommentar sektion på widget-tilpasningssiden, der slår realtidsopdateringer af tråden fra'; title='Deaktiver Live Kommentar' app-screenshot-end]

---