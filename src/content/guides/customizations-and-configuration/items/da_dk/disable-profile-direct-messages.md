[related-parameter-start name = 'disableProfileDirectMessages'; type = 'boolean'; related-parameter-end]

Som standard vil FastComments vise en "Direkte beskeder" faneblad på brugerprofiler, så besøgende kan sende direkte beskeder til en bruger.

Vi kan dog deaktivere dette faneblad:

[code-example-start config = {disableProfileDirectMessages: true}; linesToHighlight = [6]; title = 'Deaktiver direkte beskeder på profil'; code-example-end]

Dette kan også gøres uden kode. På widget-tilpasningssiden, se sektionen "Deaktiver direkte beskeder".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-direct-messages']; selector = '.disable-profile-direct-messages'; alt='Widget-tilpasningsside med afkrydsningsfeltet Deaktiver direkte beskeder markeret for at skjule fanebladet med profilbeskeder'; title='Deaktiver direkte beskeder på profil' app-screenshot-end]