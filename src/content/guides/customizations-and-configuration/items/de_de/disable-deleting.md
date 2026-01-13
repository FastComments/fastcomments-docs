---
Standardmäßig erlaubt FastComments Benutzern, ihre Kommentare zu löschen.

Es ist jedoch möglich, dies zu verhindern.

Auf der Seite zur Anpassung des Widgets finden Sie die Option "Löschen deaktivieren".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; title='Disable Comment Deleting' app-screenshot-end]

- Dies betrifft nur normale Kommentatoren und nicht Moderatoren oder Administratoren, die weiterhin löschen können.
- Dies wirkt sich auch auf API-Integrationen aus, wenn `contextUserId` übergeben wird. 

---