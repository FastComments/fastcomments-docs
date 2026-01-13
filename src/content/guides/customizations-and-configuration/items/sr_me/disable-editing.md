---
По подразумеваној поставци, FastComments ће дозволити корисницима да уређују своје коментаре.

Међутим, ово је могуће онемогућити.

На страници за прилагођавање виџета, погледајте опцију „Онемогући уређивање“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-edit'; selector = '.disable-commenter-comment-edit'; title='Disable Comment Editing' app-screenshot-end]

- Ово утиче само на обичне коментаторе и не утиче на модераторе или администраторе, који ће и даље моћи да уређују.
- Ово ће такође утицати на API интеграције када се проследи `contextUserId`. 

---