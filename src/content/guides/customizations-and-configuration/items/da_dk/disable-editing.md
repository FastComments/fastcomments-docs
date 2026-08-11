---
Som standard vil FastComments tillade brugere at redigere deres kommentarer.

Det er dog muligt at forhindre dette.

På widget-tilpasningssiden, se indstillingen "Disable Editing".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-edit'; selector = '.disable-commenter-comment-edit'; alt='Deaktiver redigering-indstillingen på widget-tilpasningssiden, som forhindrer kommentatorer i at redigere deres kommentarer'; title='Deaktiver kommentarredigering' app-screenshot-end]

- Dette påvirker kun almindelige kommentatorer og ikke moderatorer eller administratorer, som stadig vil kunne redigere.
- Dette vil også påvirke API-integrationer, når `contextUserId` bliver videregivet. 

---