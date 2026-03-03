[related-parameter-start name = 'disableProfileComments'; type = 'boolean'; related-parameter-end]

Som standard viser FastComments en fane med "Profilkommentarer" på brugerprofiler, så besøgende kan skrive kommentarer på en persons profil.

Men vi kan deaktivere denne fane:

[code-example-start config = {disableProfileComments: true}; linesToHighlight = [6]; title = 'Disable Profile Comments'; code-example-end]

Dette kan også gøres uden kode. På widgetens tilpasningsside, se afsnittet "Deaktiver profilkommentarer".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-comments']; selector = '.disable-profile-comments'; title='Disable Profile Comments' app-screenshot-end]

---