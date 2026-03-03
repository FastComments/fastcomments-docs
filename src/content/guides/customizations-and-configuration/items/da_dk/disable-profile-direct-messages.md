[related-parameter-start name = 'disableProfileDirectMessages'; type = 'boolean'; related-parameter-end]

Som standard viser FastComments fanen "Direkte beskeder" på brugerprofiler, så besøgende kan sende direkte beskeder til en bruger.

Dog kan vi deaktivere denne fane:

[code-example-start config = {disableProfileDirectMessages: true}; linesToHighlight = [6]; title = 'Disable Profile Direct Messages'; code-example-end]

Det kan også gøres uden kode. På siden til tilpasning af widgetten, se afsnittet "Deaktiver direkte beskeder".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-direct-messages']; selector = '.disable-profile-direct-messages'; title='Disable Profile Direct Messages' app-screenshot-end]