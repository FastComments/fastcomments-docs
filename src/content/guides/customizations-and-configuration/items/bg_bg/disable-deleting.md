---
По подразбиране FastComments позволява на потребителите да изтриват коментарите си.

Въпреки това е възможно това да бъде предотвратено.

В страницата за персонализиране на уиджета вижте опцията "Изключване на изтриването".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; title='Disable Comment Deleting' app-screenshot-end]

- Това засяга само обикновените Commenters и не засяга модераторите или администраторите, които все още ще могат да изтриват.
- Това ще засегне и API интеграциите, когато бъде предаден `contextUserId`. 

---