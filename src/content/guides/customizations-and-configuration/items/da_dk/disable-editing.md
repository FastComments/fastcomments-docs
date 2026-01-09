---
Som standard tillader FastComments brugere at redigere deres kommentarer.

Det er dog muligt at forhindre dette.

På siden til tilpasning af widgeten skal du se indstillingen "Deaktiver redigering".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-edit'; selector = '.disable-commenter-comment-edit'; title='Disable Comment Editing' app-screenshot-end]

- Dette påvirker kun almindelige kommentatorer og ikke moderatorer eller administratorer, som stadig vil kunne redigere.
- Dette vil også påvirke API-integrationer, når `contextUserId` sendes. 

---