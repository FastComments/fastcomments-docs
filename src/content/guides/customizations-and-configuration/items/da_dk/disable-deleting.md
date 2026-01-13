---
Som standard tillader FastComments brugere at slette deres kommentarer.

Det er dog muligt at forhindre dette.

På siden til tilpasning af widgeten, se indstillingen "Deaktiver sletning".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; title='Disable Comment Deleting' app-screenshot-end]

- Dette påvirker kun almindelige kommentatorer og ikke moderatorer eller administratorer, som stadig vil kunne slette.
- Dette vil også påvirke API-integrationer, når `contextUserId` angives. 

---