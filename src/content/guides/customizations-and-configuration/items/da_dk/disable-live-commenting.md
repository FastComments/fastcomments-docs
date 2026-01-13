[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Som standard har FastComments live-kommentering aktiveret.

Det betyder, at alle seere af kommentartråden bør se det samme indhold.

For eksempel, hvis en kommentar tilføjes, skal den vises. Hvis en kommentar redigeres eller fjernes,
vil den blive redigeret eller fjernet for alle seere af tråden. Det samme gælder for stemmer og alle moderationshandlinger.

Du kan dog deaktivere dette:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Dette kan også gøres uden kode. På widget-tilpasningssiden, se afsnittet "Deaktiver live-kommentering".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]