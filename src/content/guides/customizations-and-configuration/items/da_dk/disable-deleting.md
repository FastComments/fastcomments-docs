---
Som standard tillader FastComments brugere at slette deres kommentarer.

Det er dog muligt at forhindre dette.

På widget-tilpasningssiden kan du se indstillingen "Disable Deleting".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; alt='Disable Deleting-indstillingen på widget-tilpasningssiden, som forhindrer kommentatorer i at fjerne deres kommentarer'; title='Deaktivér sletning af kommentarer' app-screenshot-end]

- Dette påvirker kun almindelige kommentatorer og ikke moderatorer eller administratorer, som stadig vil kunne slette.
- Dette vil også påvirke API-integrationer, når `contextUserId` sendes. 

---