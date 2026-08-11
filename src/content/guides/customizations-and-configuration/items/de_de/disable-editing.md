---
Standardmäßig erlaubt FastComments den Benutzern, ihre Kommentare zu bearbeiten.

Es ist jedoch möglich, dies zu verhindern.

Auf der Seite zur Widget-Anpassung finden Sie die Option „Bearbeitung deaktivieren“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-edit'; selector = '.disable-commenter-comment-edit'; alt='Option „Bearbeitung deaktivieren“ auf der Widget-Anpassungsseite, die verhindert, dass Kommentatoren ihre Kommentare bearbeiten'; title='Deaktivieren der Kommentarbearbeitung' app-screenshot-end]

- Dies betrifft nur reguläre Kommentatoren und nicht Moderatoren oder Administratoren, die weiterhin bearbeiten können.
- Dies wirkt sich auch auf API-Integrationen aus, wenn `contextUserId` übergeben wird. 

---